mod board;
mod moves;
mod alphabeta;

use crate::moves::Move;
use crate::alphabeta::evaluate_board;


// TODO: implement compatability with the bash script so that we can actually play
// TODO: Improve generation of valid moves, currently running out of moves -> panic
fn main() {
    let board_string = "eeeeeeee\neeeeeeee\neeeeeeee\neeewbeee\neeebweee\neeeeeeee\neeeeeeee\neeeeeeee";
    let initial_board = board::Board::read(board_string);
    let next_board = alphabeta::find_next_board(board_string);

    println!("Initial board: ");
    initial_board.print();
    println!("Proposed next board: ");
    next_board.print();

}
