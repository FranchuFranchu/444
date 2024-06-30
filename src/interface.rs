use wasm_bindgen::prelude::*;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};

use crate::state::State;

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
        Self::default()
    }
    pub fn from_base64(&mut self, b64: &str) -> Result<(), String> {
        let n = URL_SAFE.decode(b64).unwrap();
        let length = u32::from_le_bytes((&n[0..4]).try_into().unwrap()) as usize;
        dbg!(&n);
        self.add_moves(&n[4..], length)
    }
    pub fn to_base64(&mut self) -> String {
        let mut v = vec![];
        v.extend((self.history_len() as u32).to_le_bytes().into_iter());
        assert!(v.len() == 4);
        v.extend(self.export_moves().into_iter());
        dbg!(&v);
        URL_SAFE.encode(v)
    }
    pub fn add_moves(&mut self, array: &[u8], len: usize) -> Result<(), String> {
        for index in 0..len {
            let n = (array[index / 2] >> ((index % 2) * 4)) % 16;
            self.play_at(n % 4, n / 4)?;
        };
        Ok(())
    }
    pub fn export_moves(&mut self) -> Box<[u8]> {
        let mut arr = vec![];
        for (index, (x, y)) in self.move_history.iter().enumerate() {
            if index % 2 == 0 {
                arr.push(x | y << 2);
            } else {
                let a = arr.last_mut().unwrap();
                *a |= (x << 4) | (y << 6)
            }
        }
        arr.into_boxed_slice()
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
        let new_state = state
            .cond_flip(team)
            .choose_next(diff)
            .map(|x| x.cond_flip(team));
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
        self.history.pop()
    }
    pub fn undo_to(&mut self, len: usize) {
        self.history.truncate(len);
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
