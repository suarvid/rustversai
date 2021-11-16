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

    println!("Initial board: ");
    initial_board.print();

    let main_diagonal_left_right = board::get_diagonal_left_to_right(&initial_board, 7);
    println!("Main left-to-right diagonal of initial board: ");
    for elem in main_diagonal_left_right {
        println!("{}", elem);
    }
    let main_diagonal_right_left = board::get_diagonal_right_to_left(&initial_board, 0);
    println!("Main right-to-left diagonal of initial board: ");
    for elem in main_diagonal_right_left {
        println!("{}", elem);
    }
}
