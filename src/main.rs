mod alphabeta;
mod move_generator;
use alphabeta::alphabeta_at_root;

use crate::alphabeta::{VERY_HIGH, VERY_LOW, alphabeta_move_gen, evaluate_board, generate_children};
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
// TODO: Same moves are always made, even with different evaluation functions
// TODO: Look over the alpha-beta algorithm step by step, read Anton's comment in the discussion about passing a copy etc.
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
    // TODO: DOES NOT SEEM TO MATTER WHAT I DO, THE OTHER AI ALWAYS WINS WITH THE SAME AMOUNT OF POINTS 
    // IF WE PLAY AS BLACK, WHITE ALWAYS WINS WITH 4 POINTS
    // IF WE PLAY AS WHITE, BLACK ALWAYS WINS WITH 24 POINTS, I.E. THE SAME MOVES ALWAYS SEEM TO BE SELECTED!
    /*println!("Evaluated value of board:");
    println!("{}", evaluate_board(&board));*/
    //TODO: Try alpha-beta on single boards to see what the resulting value is
    /* let response = match alphabeta_at_root(&board, 16) {
        Some(m) => format!("({},{})", m.row, m.col),
        None => String::from("pass")
    };
    print!("{}", response); */
    let response = match alphabeta_move_gen(&board, start_time, time_limit) {
        Some(m) => format!("({},{})", m.row, m.col),
        None => String::from("pass")
    };

    print!("{}", response);
   
}