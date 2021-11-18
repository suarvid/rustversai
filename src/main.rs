mod alphabeta;
mod move_generator;
use crate::alphabeta::alphabeta_move_gen;
use crate::move_generator::{get_move_from_board_diff, get_moves, Move, OthelloPosition};
use std::env;
use std::thread;
use std::time;

// TODO: Find out if this is okay, does allow 100000 nodes to be explored
// const STACK_SIZE: usize = 32 * 1024 * 1024;

/*
fn main() {
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    child.join().unwrap();
}*/
// TODO: BOOOOOOOOOOOOOOOOOOOOOOOOOOOXOOOOOOOOOOOOOXOOOOOOOXOOOOOOOXEEEOOO returns (0,0) instantly, figure out why!
fn main() {

   /*  let worst_white = move_generator::OthelloPosition::worst_for_max();
    let worst_black = move_generator::OthelloPosition::worst_for_min();

    let worst_white_score = alphabeta::evaluate_board(&worst_white);
    let worst_black_score = alphabeta::evaluate_board(&worst_black);

    println!("Score for worst white board: {}", worst_white_score);
    println!("Score for worst black board: {}", worst_black_score); */
    
    let args: Vec<String> = env::args().collect();
    let start_time = time::Instant::now();
    // call for move is $white $pos $time
    let pos_string = &args[1];
    let time_limit = (&args[2]).parse::<u64>().unwrap(); //TODO: Implement this, start by assuming time limit = 1s
    let board = OthelloPosition::new(pos_string);

    let response = match alphabeta_move_gen(&board, start_time, time_limit) {
        Some(m) => format!("({},{})", m.row, m.col),
        None => String::from("pass")
    };

    print!("{}", response);
   
}