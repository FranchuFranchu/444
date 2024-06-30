use wasm_bindgen::prelude::*;

use crate::state::State;

#[wasm_bindgen]
#[derive(Default)]
pub struct Interface {
    history: Vec<State>,
}

#[wasm_bindgen]
pub struct WinResult(pub bool, pub State);

#[wasm_bindgen]
impl Interface {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
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
        let state = state
            .cond_flip(team)
            .choose_next(diff)
            .map(|x| x.cond_flip(team));
        if let Some(state) = state {
            self.history.push(state);
            Ok(())
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
