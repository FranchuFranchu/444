#![feature(const_trait_impl, iter_from_coroutine, coroutines, coroutine_clone)]

use std::{collections::HashSet, f64::{INFINITY, NEG_INFINITY}, io::{Read, Write}, ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr}};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::state::State;

pub mod state;
pub mod interface;

fn main_2() {

    for depth in 0.. {
        let mut set = HashSet::new();
        for i in State::empty().recurse_children(depth) {
            set.insert(i);
        }
        println!("{:?} movimientos, un equipo: {:?}", depth + 1, set.len());
        let mut set = HashSet::new();
        for i in State::empty().recurse_children_flip(depth) {
            set.insert(i);
        }
        println!("{:?} movimientos, dos equipos: {:?}", depth + 1, set.len());
    }
}