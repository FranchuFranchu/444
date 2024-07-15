use std::time::Instant;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use wasm_bindgen::prelude::*;

use crate::state::State;

fn difficulty_to_visit(diff: isize) -> Option<usize> {
    // it's roughly linear:

    // 100000 -> 4392ms
    // 10000 -> 370ms
    // 1000 -> 36ms
    // base difficulty will take 1 second on my machine.
    if diff < 0 {
        (22768u64 >> -diff).try_into().ok()
    } else {
        (22768u64 << diff).try_into().ok()
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Interface {
    history: Vec<State>,
    move_history: Vec<(u8, u8)>,
}

#[wasm_bindgen]
pub struct WinResult(pub bool, pub State);

#[wasm_bindgen]
impl Interface {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        if cfg!(all(target_arch = "wasm32", target_os = "unknown")) {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        }
        Self::default()
    }
    pub fn from_base64(&mut self, b64: &str) -> Result<(), String> {
        let n = URL_SAFE.decode(b64).map_err(|x| format!("{}", x))?;
        let length =
            u32::from_le_bytes((&n[0..4]).try_into().map_err(|x| format!("{}", x))?) as usize;
        self.add_moves(&n[4..], length)
    }
    pub fn to_base64(&mut self) -> Option<String> {
        let mut v = vec![];
        v.extend((self.history_len() as u32).to_le_bytes().into_iter());
        assert!(v.len() == 4);
        v.extend(self.export_moves()?.into_iter());
        Some(URL_SAFE.encode(v))
    }
    pub fn add_moves(&mut self, array: &[u8], len: usize) -> Result<(), String> {
        for index in 0..len {
            let n = (array[index / 2] >> ((index % 2) * 4)) % 16;
            self.play_at(n % 4, n / 4)?;
        }
        println!(
            "{:?}",
            self.move_history
                .iter()
                .map(|(a, b)| format!("{:x}", a + b * 4))
                .collect::<Vec<_>>()
        );
        Ok(())
    }
    pub fn export_moves(&mut self) -> Option<Box<[u8]>> {
        let mut arr = vec![];
        for (index, (x, y)) in self.move_history.iter().enumerate() {
            if index % 2 == 0 {
                arr.push(x | y << 2);
            } else {
                let a = arr.last_mut()?;
                *a |= (x << 4) | (y << 6)
            }
        }
        Some(arr.into_boxed_slice())
    }
    pub fn history_len(&self) -> usize {
        self.history.len()
    }
    pub fn get_last(&self) -> State {
        self.history.last().map(|x| *x).unwrap_or(State::empty())
    }
    pub fn player_id(&self) -> bool {
        self.history.len() % 2 == 0
    }
    pub fn play_ai(&mut self, diff: isize) -> Result<(), String> {
        let team = self.player_id();
        let state = self.get_last();
        let visit_amount = difficulty_to_visit(diff).ok_or("Invalid difficulty".to_string())?;
        let t = if cfg!(all(target_arch = "wasm32", target_os = "unknown")) {
            None
        } else {
            Some(Instant::now())
        };

        let new_state = state
            .cond_flip(team)
            .choose_next(visit_amount)
            .map(|x| x.cond_flip(team));
        if !cfg!(all(target_arch = "wasm32", target_os = "unknown")) {
            let t2 = Instant::now();
            println!("{}: Took {}ms", visit_amount, (t2 - t.unwrap()).as_millis());
        }

        if let Some(new_state) = new_state {
            let m = state.get_move(new_state);
            self.play_at(m.0, m.1)
        } else {
            Err("No available options!".to_string())
        }
    }
    pub fn play_at(&mut self, x: u8, y: u8) -> Result<(), String> {
        let team = self.player_id();
        self.history.push(
            self.get_last()
                .play_at_team(team, x, y)
                .ok_or("it's full! :(".to_string())?,
        );
        self.move_history.push((x, y));
        Ok(())
    }
    pub fn push(&mut self, state: State) {
        self.history.push(state)
    }
    pub fn undo(&mut self) -> Option<State> {
        self.move_history.pop();
        self.history.pop()
    }
    pub fn undo_to(&mut self, len: usize) {
        self.history.truncate(len);
        self.move_history.truncate(len);
    }
    pub fn peek(&mut self, step: usize) -> Option<State> {
        self.history.get(step).map(|x| *x)
    }
    pub fn winner(&mut self) -> Option<WinResult> {
        let state = self.get_last();
        if let Some(winner) = state.winner() {
            println!("winner: {}", winner);

            for mut winstate in State::lines() {
                let mut state = state;
                if !winner {
                    state = state.flip_team()
                }
                winstate.1 = u64::MAX;
                if (winstate & state).count_own() == 4 {
                    if !winner {
                        winstate = winstate.flip_team();
                    }
                    return Some(WinResult(winner, winstate));
                }
            }
            todo!();
        } else {
            None
        }
    }
}
