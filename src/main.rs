mod alphabeta;
mod board;
mod evaluator;
mod move_generator;

use crate::alphabeta::alphabeta_move_gen;
use crate::board::OthelloPosition;
use std::env;
use std::time;

fn main() {

    let args: Vec<String> = env::args().collect();
    let start_time = time::Instant::now();
    let pos_string = &args[1];
    let time_limit = (&args[2]).parse::<u64>().unwrap(); 
    let board = OthelloPosition::new(pos_string);
    let response = match alphabeta_move_gen(&board, start_time, time_limit) {
        Some(m) => format!("({},{})", m.row, m.col),
        None => String::from("pass"),
    };

    print!("{}", response);
}
