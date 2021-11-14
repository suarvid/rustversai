mod board;
mod moves;
mod alphabeta;

use crate::moves::Move;
use crate::alphabeta::evaluate_board;


// TODO: implement compatability with the bash script so that we can actually play
fn main() {
    let board_string = "eeeeeeee\neeeeeeee\neeeeeeee\neeewbeee\neeebweee\neeeeeeee\neeeeeeee\neeeeeeee";
    let board = board::Board::read(board_string);


    println!("Initial board: ");
    board.print();


}
