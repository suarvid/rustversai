mod move_generator;
mod alphabeta;
use crate::move_generator::{OthelloPosition, Move, getMoves, get_move_from_board_diff};
use crate::alphabeta::alphabeta_move_gen;
use std::env;
// TODO: implement compatability with the bash script so that we can actually play
// TODO: Improve generation of valid moves, currently running out of moves -> panic
fn main() {
    let args: Vec<String> = env::args().collect();

    // Just for testing
    if args.len() == 1{
        let board_string = "WEEEEEEEEEEEEEEEEEEEEEEEEEEEOXEEEEEEXOEEEEEEEEEEEEEEEEEEEEEEEEEEE";
        let recommended_move = get_best_move(board_string);
        println!("Recommended move is: {:?}", recommended_move);
    }
    

    // call for move is $white $pos $time
    let pos_string = &args[1];
    let time_limit = &args[2]; //TODO: Implement this, start by assuming time limit = 1s
    print!("{}", get_best_move(&pos_string[..]));

}


fn get_best_move(input_string: &str) -> String {
    let current_board = move_generator::OthelloPosition::new(input_string);
    let next_board = find_next_board(input_string);
    println!("Next board is: ");
    println!("{:?}", next_board); 
    match move_generator::get_move_from_board_diff(&current_board, &next_board) {
        Some(m) => format!("({},{})", m.row, m.col),
        None => panic!("No move found! Arvid messed up!")
    }
}

fn find_next_board(input_string: &str) -> OthelloPosition {
    assert!(input_string.is_ascii()); // should be reasonable
    let board_string = input_string.chars().collect::<String>();
    let board = OthelloPosition::new(&board_string[..]); // takes a &str, not a String
    match alphabeta_move_gen(&board) {
        Some(m) => match Move::make_move(&board, &Some(m)) {
            Some(b) => return b,
            None => panic!("No move found for board and player!"),
        },
        None => panic!("No move found for board and player!"),
    }
}