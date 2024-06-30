use std::{
    f64::{INFINITY, NEG_INFINITY},
    ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr},
};

use wasm_bindgen::prelude::*;
/// Layout:
/// [     z = 3    ][     z = 2    ][     z = 1    ][     z = 0    ] z
/// [y3][y2][y1][y0][y3][y2][y1][y0][y3][y2][y1][y0][y3][y2][y1][y0] y
/// 3210321032103210321032103210321032103210321032103210321032103210 x
#[wasm_bindgen]
#[derive(Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct State(pub u64, pub u64);

impl const BitAnd for State {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0, self.1 & rhs.1)
    }
}
impl const BitAnd<u64> for State {
    type Output = Self;

    fn bitand(self, rhs: u64) -> Self::Output {
        Self(self.0 & rhs, self.1 & rhs)
    }
}
impl const BitOr for State {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0, self.1 | rhs.1)
    }
}
impl const BitXor for State {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0, self.1 ^ rhs.1)
    }
}
impl const Not for State {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0, !self.1)
    }
}
impl const Shl<u64> for State {
    type Output = Self;

    fn shl(self, rhs: u64) -> Self::Output {
        Self(self.0 << rhs, self.1 << rhs)
    }
}
impl const Shr<u64> for State {
    type Output = Self;

    fn shr(self, rhs: u64) -> Self::Output {
        Self(self.0 >> rhs, self.1 >> rhs)
    }
}

impl core::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut p = self.0;
        let mut v = self.1;
        for i in 0..64 {
            if i != 0 {
                if i % 16 == 0 {
                    f.write_str(" ")?;
                }
                if i % 4 == 0 {
                    f.write_str(" ")?;
                }
            }
            f.write_str(match (p & 1, v & 1) {
                (0, 0) => "-",
                (0, 1) => "-",
                (1, 0) => "0",
                (1, 1) => "1",
                _ => todo!(),
            })?;
            p >>= 1;
            v >>= 1;
        }
        Ok(())
    }
}

#[wasm_bindgen]
impl State {
    #[wasm_bindgen(constructor)]
    pub fn empty() -> Self {
        Self::default()
    }
    pub fn wasm_clone(&self) -> Self {
        self.clone()
    }
    pub fn get_at(self, x: u8, y: u8, z: u8) -> Option<bool> {
        let s = self & Self::xyz_mask(x, y, z);
        match (s.count_own() > 0, s.flip_team().count_own() > 0) {
            (false, true) => Some(false),
            (true, false) => Some(true),
            _ => None,
        }
    }
    pub fn set_at(self, x: u8, y: u8, z: u8, val: Option<bool>) -> State {
        let p = Self::xyz_mask(x, y, z).0;
        match val {
            Some(true) => self | Self(p, p),
            Some(false) => self & Self(p, !p) | Self(p | self.0, 0),
            None => self & Self(!p, !p),
        }
    }
}
impl State {
    pub const fn flip_team(self) -> Self {
        Self(self.0, !self.1)
    }
    const fn pl_mask(pl: u64) -> Self {
        Self(pl, u64::MAX)
    }
    pub fn x_mask(x: u8) -> Self {
        Self::pl_mask(0x1111_1111_1111_1111) << x.into()
    }
    pub fn y_mask(y: u8) -> Self {
        Self::pl_mask(0x000F_000F_000F_000F) << (y * 4).into()
    }
    pub fn z_mask(z: u8) -> Self {
        Self::pl_mask(0x0000_0000_0000_FFFF) << (z * 16).into()
    }
    pub fn xyz_mask(x: u8, y: u8, z: u8) -> Self {
        Self::x_mask(x) & Self::y_mask(y) & Self::z_mask(z)
    }
    pub fn count_own(self) -> u64 {
        (self.0 & self.1).count_ones() as u64
    }
    pub fn xy_diag_mask_1(z: u8) -> Self {
        Self::pl_mask(0b1000_0100_0010_0001) << (z * 16).into()
    }
    pub fn xy_diag_mask_2(z: u8) -> Self {
        Self::pl_mask(0b0001_0010_0100_1000) << (z * 16).into()
    }
    pub fn is_empty(self) -> bool {
        self.0.count_ones() == 0
    }
    pub fn lines() -> impl Iterator<Item = Self> {
        std::iter::from_coroutine(move || {
            for x in 0..4 {
                for y in 0..4 {
                    yield Self::x_mask(x) & Self::y_mask(y);
                }
            }
            for x in 0..4 {
                for z in 0..4 {
                    yield Self::x_mask(x) & Self::z_mask(z);
                }
            }
            for y in 0..4 {
                for z in 0..4 {
                    yield Self::y_mask(y) & Self::z_mask(z);
                }
            }
            let x_z_diag = (0..4)
                .zip(0..4)
                .map(|(x, y)| Self::x_mask(x) & Self::z_mask(y))
                .reduce(|a, b| a | b)
                .unwrap();
            let x_z_diag_anti = (0..4)
                .zip((0..4).rev())
                .map(|(x, y)| Self::x_mask(x) & Self::z_mask(y))
                .reduce(|a, b| a | b)
                .unwrap();
            let x_y_diag = (0..4)
                .zip(0..4)
                .map(|(x, y)| Self::x_mask(x) & Self::y_mask(y))
                .reduce(|a, b| a | b)
                .unwrap();
            let x_y_diag_anti = (0..4)
                .zip((0..4).rev())
                .map(|(x, y)| Self::x_mask(x) & Self::y_mask(y))
                .reduce(|a, b| a | b)
                .unwrap();
            let y_z_diag = (0..4)
                .zip(0..4)
                .map(|(x, y)| Self::y_mask(x) & Self::z_mask(y))
                .reduce(|a, b| a | b)
                .unwrap();
            let y_z_diag_anti = (0..4)
                .zip((0..4).rev())
                .map(|(x, y)| Self::y_mask(x) & Self::z_mask(y))
                .reduce(|a, b| a | b)
                .unwrap();
            for z in 0..4 {
                yield Self::z_mask(z) & x_y_diag;
                yield Self::z_mask(z) & x_y_diag_anti;
            }
            for y in 0..4 {
                yield Self::y_mask(y) & x_z_diag;
                yield Self::y_mask(y) & x_z_diag_anti;
            }
            for x in 0..4 {
                yield Self::x_mask(x) & y_z_diag;
                yield Self::x_mask(x) & y_z_diag_anti;
            }
            yield (0..4)
                .zip(0..4)
                .zip(0..4)
                .map(|((x, y), z)| Self::xyz_mask(x, y, z))
                .reduce(|a, b| a | b)
                .unwrap();
            yield ((0..4).rev())
                .zip(0..4)
                .zip(0..4)
                .map(|((x, y), z)| Self::xyz_mask(x, y, z))
                .reduce(|a, b| a | b)
                .unwrap();
            yield (0..4)
                .zip((0..4).rev())
                .zip(0..4)
                .map(|((x, y), z)| Self::xyz_mask(x, y, z))
                .reduce(|a, b| a | b)
                .unwrap();
            yield ((0..4).rev())
                .zip((0..4).rev())
                .zip(0..4)
                .map(|((x, y), z)| Self::xyz_mask(x, y, z))
                .reduce(|a, b| a | b)
                .unwrap();
            ()
        })
    }
    pub fn play_at(self, x: u8, y: u8) -> Option<State> {
        for z in 0..4 {
            if (self & Self::xyz_mask(x, y, z)).is_empty() {
                return Some(self | Self(Self::xyz_mask(x, y, z).0, Self::xyz_mask(x, y, z).0));
            }
        }
        None
    }
    pub fn cond_flip(self, team: bool) -> State {
        if team {
            self.flip_team()
        } else {
            self
        }
    }
    pub fn play_at_team(self, team: bool, x: u8, y: u8) -> Option<State> {
        self.cond_flip(team)
            .play_at(x, y)
            .map(|x| x.cond_flip(team))
    }
    pub fn children_own(self) -> impl Iterator<Item = Self> {
        std::iter::from_coroutine(move || {
            for x in 0..4 {
                for y in 0..4 {
                    if let Some(e) = self.play_at(x, y) {
                        yield e;
                    }
                }
            }
        })
    }
    pub fn children_they(self) -> impl Iterator<Item = Self> {
        self.flip_team().children_own().map(|x| x.flip_team())
    }
    pub fn recurse_children(self, depth: usize) -> impl Iterator<Item = Self> {
        std::iter::from_coroutine(Box::new(move || {
            for i in self.children_own() {
                if depth == 0 {
                    yield i;
                } else {
                    for j in i.recurse_children(depth - 1) {
                        yield j;
                    }
                }
            }
        }))
    }
    pub fn recurse_children_flip(self, depth: usize) -> impl Iterator<Item = Self> {
        std::iter::from_coroutine(Box::new(move || {
            for i in self.flip_team().children_own() {
                if depth == 0 {
                    yield i;
                } else {
                    for j in i.recurse_children_flip(depth - 1) {
                        yield j;
                    }
                }
            }
        }))
    }
    pub fn score(self, eval: &impl Fn(State) -> f64, depth: usize, own: bool) -> f64 {
        if self.did_we_win() {
            return INFINITY;
        }
        if self.flip_team().did_we_win() {
            return -INFINITY;
        }
        if depth == 0 {
            return eval(self);
        } else {
            if own {
                self.children_own()
                    .map(|x| x.score(eval, depth - 1, !own))
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap()
            } else {
                self.children_they()
                    .map(|x| x.score(eval, depth - 1, !own))
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap()
            }
        }
    }
    pub fn did_we_win(self) -> bool {
        for i in Self::lines() {
            let line = self & i;
            if line.count_own() == 4 {
                return true;
            }
        }
        return false;
    }
    pub fn eval(self) -> f64 {
        let mut tot = 0f64;
        for i in Self::lines() {
            let line = self & i;
            let n_our = line.count_own();
            let n_they = line.flip_team().count_own();
            let score = match (n_our, n_they) {
                (0, 0) => 0.,
                (0, 1) => -1.,
                (0, 2) => -2.,
                (0, 3) => -4.,
                (0, 4) => NEG_INFINITY,
                (1, 0) => 1.,
                (2, 0) => 2.,
                (3, 0) => 4.,
                (4, 0) => INFINITY,
                (_, _) => 0.,
            };
            tot += score;
        }
        tot
    }
    pub fn get_move(self, next: State) -> (u8, u8) {
        let n = (next.0 & !self.0);
        let n = n.ilog2() as u8 % 16;
        (n % 4, n / 4)
    }
    pub fn choose_next(self, diff: isize) -> Option<State> {
        let dep = if self.count_own() < 10 {
            4 + diff
        } else if self.count_own() < 15 {
            5 + diff
        } else if self.count_own() < 25 {
            6 + diff
        } else {
            7 + diff
        };
        self.children_own().max_by(|a, b| {
            a.score(&|x| x.eval(), dep.unsigned_abs(), false)
                .partial_cmp(&b.score(&|x| x.eval(), dep.unsigned_abs(), false))
                .unwrap()
        })
    }
    pub fn winner(self) -> Option<bool> {
        if self.did_we_win() {
            Some(true)
        } else if self.flip_team().did_we_win() {
            Some(false)
        } else {
            None
        }
    }
    pub fn pretty(self) -> String {
        let mut s = String::new();
        for y in 0..4 {
            for z in 0..4 {
                for x in 0..4 {
                    let mask = self & Self::xyz_mask(x, y, z);
                    s.push_str(
                        match (mask.count_own() > 0, mask.flip_team().count_own() > 0) {
                            (false, false) => "-",
                            (false, true) => "O",
                            (true, false) => "X",
                            _ => todo!(),
                        },
                    );
                }
                s.push_str(" ");
            }
            s.push_str("\n");
        }
        s
    }
}
