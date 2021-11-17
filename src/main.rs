mod alphabeta;
mod move_generator;
use crate::alphabeta::alphabeta_move_gen;
use crate::move_generator::{get_move_from_board_diff, get_moves, Move, OthelloPosition};
use std::env;
use std::thread;
// TODO: implement compatability with the bash script so that we can actually play
// TODO: Improve generation of valid moves, currently running out of moves -> panic

// TODO: Find out if this is okay, does allow 100000 nodes to be explored
const STACK_SIZE: usize = 32 * 1024 * 1024;

fn main() {
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    child.join().unwrap();
}

fn run() {
    let args: Vec<String> = env::args().collect();

    // call for move is $white $pos $time
    let pos_string = &args[1];
    let time_limit = (&args[2]).parse::<u64>().unwrap(); //TODO: Implement this, start by assuming time limit = 1s
    print!("{}", get_best_move(&pos_string[..], time_limit));
}

fn get_best_move(input_string: &str, time_limit: u64) -> String {
    let current_board = move_generator::OthelloPosition::new(input_string);
    match find_next_board(input_string, time_limit) {
        Some(nb) => {
            match move_generator::get_move_from_board_diff(&current_board, &nb) {
                Some(m) => format!("({},{})", m.row + 1, m.col + 1), //TODO: Read reason in OthelloPosition.java
                None => panic!("No move found! Arvid messed up!"),
            }
        }
        None => String::from("pass"),
    }
}

fn find_next_board(input_string: &str, time_limit: u64) -> Option<OthelloPosition> {
    assert!(input_string.is_ascii()); // should be reasonable
    let board_string = input_string.chars().collect::<String>();
    let board = OthelloPosition::new(&board_string[..]); // takes a &str, not a String
    match alphabeta_move_gen(&board, time_limit) {
        Some(m) => match Move::make_move(&board, &Some(m)) {
            Some(b) => return Some(b),
            None => None,
        },
        None => None,
    }
}
