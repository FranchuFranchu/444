#![feature(const_trait_impl, iter_from_coroutine, coroutines, coroutine_clone)]

use std::collections::HashSet;

use crate::state::State;

pub mod interface;
pub mod state;

fn _count() {
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

pub fn print_lines() {
    for i in State::lines() {
        print!("State({:#018x}, {:#018x}),\n", i.0, i.1)
    }
}
