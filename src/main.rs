#![feature(const_trait_impl, iter_from_coroutine, coroutines, coroutine_clone)]

pub mod interface;
pub mod state;

use std::io::Write;

use crate::state::State;

use crate::interface::{Interface, WinResult};

fn main() {
    let mut interface = Interface::new();
    let mut ai_turns = 0;
    loop {
        let args: Vec<String> = if ai_turns == 0 {
            print!("> ");
            std::io::stdout().flush().unwrap();
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            let mut i = s.chars();
            vec![i.next().unwrap().to_string(), i.collect()]
        } else {
            ai_turns -= 1;
            vec!["a".to_string()]
        };

        if args[0] == "a" {
            if args.len() > 1 && args[1].trim().len() > 0 {
                let turns: u64 = args[1].trim().parse().unwrap();
                ai_turns += turns;
            };
            interface.play_ai(0).unwrap();
        }
        if args[0] == "p" {
            let n = u8::from_str_radix(&args[1].trim(), 16).unwrap();
            interface.play_at(n % 4, n / 4).unwrap();
        }
        if args[0] == "u" {
            let rt = if args.len() > 1 && args[1].trim().len() > 0 {
                args[1].trim().parse().unwrap()
            } else {
                interface.history_len() - 1
            };
            interface.undo_to(rt);
            ai_turns = 0;
        }
        if args[0] == "q" {
            return;
        }
        println!(
            "Move: {}\n{}",
            interface.history_len(),
            interface.get_last().pretty()
        );
        if let Some(WinResult(winner, state)) = interface.winner() {
            println!("{}\n{}", winner, state.pretty());
            ai_turns = 0;
        }
    }
}
