use crate::move_generator::{get_move_from_board_diff, get_moves, Move, OthelloPosition};
use crate::move_generator::{EMPTY_CELL, PLAYER_BLACK, PLAYER_WHITE};
use std::time;
use rand::Rng;
static mut NODES_EXPANDED: u32 = 0;

// TODO: figure out if this should allow negative values
// It should probably return values in the range (-100, 100) for example
// Things that are important:
// Coin Parity
// Mobility
// Corners
// Stability
// was 14 without corners
pub fn evaluate_board(board: &OthelloPosition) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range((-100)..100) as f32
}

pub fn basic_heuristic(board: &OthelloPosition) -> isize {
    let mut white_counter = 0;
    for row in board.board {
        for c in row {
            if c == PLAYER_WHITE {
                white_counter += 1;
            }
        }
    }
    let mut black_counter = 0;
    for row in board.board {
        for c in row {
            if c == PLAYER_BLACK {
                black_counter -= 1;
            }
        }
    }

    white_counter - black_counter
}

fn coin_parity_value(board: &OthelloPosition) -> isize {
    let mut max_player_coins = 0;
    let mut min_player_coins = 0;
    for elem in &board.board {
        for c in elem {
            if *c == PLAYER_WHITE {
                max_player_coins += 1;
            } else if *c == PLAYER_BLACK {
                min_player_coins += 1;
            }
        }
    }

    100 * (max_player_coins - min_player_coins) / (max_player_coins + min_player_coins)
}

fn mobility_value(board: &OthelloPosition) -> isize {
    let num_max_moves;
    let num_min_moves;
    if board.max_player {
        // Need to create a min board
        let min_board = OthelloPosition {
            board: board.board.clone(),
            max_player: false,
        }; //Cloning might be a bit expensive
        let max_board = board;
        num_max_moves = get_moves(max_board).len() as isize;
        num_min_moves = get_moves(&min_board).len() as isize;
    } else {
        // Need to create a max board
        let max_board = OthelloPosition {
            board: board.board.clone(),
            max_player: true,
        };
        let min_board = board;
        num_max_moves = get_moves(&max_board).len() as isize;
        num_min_moves = get_moves(min_board).len() as isize;
    }

    if num_max_moves + num_min_moves != 0 {
        return (100 * (num_max_moves - num_min_moves) / (num_max_moves + num_min_moves)) as isize;
    }

    0
}

// Due to weird board formatting, corners are at [1][1], [1][8], [8][1], [8][8]
fn corners_value(board: &OthelloPosition) -> isize {
    let mut max_corners = 0;
    let mut min_corners = 0;
    let cells = &board.board;
    let top_left = cells[1][1];
    let top_right = cells[1][8];
    let bottom_left = cells[8][1];
    let bottom_right = cells[8][8];

    match top_left {
        PLAYER_WHITE => max_corners += 1,
        PLAYER_BLACK => min_corners += 1,
        _ => (),
    }
    match top_right {
        PLAYER_WHITE => max_corners += 1,
        PLAYER_BLACK => min_corners += 1,
        _ => (),
    }
    match bottom_left {
        PLAYER_WHITE => max_corners += 1,
        PLAYER_BLACK => min_corners += 1,
        _ => (),
    }
    match bottom_right {
        PLAYER_WHITE => max_corners += 1,
        PLAYER_BLACK => min_corners += 1,
        _ => (),
    }

    if max_corners + min_corners != 0 {
        return (100 * (max_corners - min_corners) / (max_corners + min_corners)) as isize;
    }

    0
}

pub fn generate_children(board: &OthelloPosition) -> Vec<OthelloPosition> {
    let possible_moves = get_moves(board);
    let mut child_boards = Vec::new();

    for p_move in possible_moves {
        match Move::make_move(&board, &Some(p_move)) {
            Some(nb) => child_boards.push(nb),
            None => continue,
        }
    }

    child_boards
}

// Maybe unnecessary to take a player, but think we always have access to one
// TODO: Rewrite this to recognize that the game is over
pub fn is_game_over(board: &OthelloPosition) -> bool {
    let mut filled_board = true;
    for row in 1..=8 {
        for col in 1..=8 {
            if board.board[row][col] == 'E' {
                // at least one empty space remaining
                filled_board = false;
                break;
            }
        }
    }
    unsafe { filled_board || NODES_EXPANDED > 100000 } //TODO: Figure out if this is OK or if we need to avoid stack overflow some other way
}

pub fn alphabeta_move_gen(
    board: &OthelloPosition,
    start_time: time::Instant,
    time_limit: u64,
) -> Option<Move> {
    let max_depth = 100;
    let mut depth_limit = 1;
    let mut best_move = None;
    let time_duration = time::Duration::new(time_limit, 0);
    while depth_limit <= max_depth
        && time::Instant::now().duration_since(start_time) < time_duration
    {
        best_move = alphabeta_at_root(board, depth_limit);
        depth_limit += 1;
    }
    // println!("Max depth reached was: {}", depth_limit);
    best_move
}

fn alphabeta_at_root(board: &OthelloPosition, depth_limit: u32) -> Option<Move> {
    // println!("In alpha_beta_at_root");
    // max wants child with max value,
    // min wants child with min value
    let mut to_beat: f32;
    let children = generate_children(board);
    // println!("Number of children found: {}", children.len());
    if board.max_player {
        let mut best_child = OthelloPosition::worst_for_max();
        to_beat = f32::NEG_INFINITY;
        if children.len() == 0 {
            // println!("length of children == 0");
            return None;
        }
        for child in children {
            // println!("Looking at child:");
            // child.print();
            // let move_to_child = get_move_from_board_diff(board, child).unwrap();
            // println!(
            // "Looking at move {},{}",
            // move_to_child.row, move_to_child.col
            // );
            let child_value = alphabeta(board, depth_limit, f32::NEG_INFINITY, f32::INFINITY);
            if child_value >= to_beat {
                to_beat = child_value;
                best_child = child;
            } else {
                // println!("{} is not greater than {}", child_value, to_beat);
            }
        }
        // let best_move = get_move_from_board_diff(board, &best_child).unwrap();
        // println!("best move is: {},{}", best_move.row, best_move.col);
        // println!("Score of best child was: {}", evaluate_board(&best_child));
        let best_move = get_move_from_board_diff(board, &best_child);
        best_move
    } else {
        let mut best_child = OthelloPosition::worst_for_min();
        to_beat = f32::INFINITY;
        if children.len() == 0 {
            return None;
        }
        for child in children {
            // println!("Looking at child:");
            // child.print();
            // let move_to_child = get_move_from_board_diff(board, &child).unwrap();
            // println!(
            // "Looking at move {},{}",
            // move_to_child.row, move_to_child.col
            // );
            let child_value = alphabeta(board, depth_limit, f32::NEG_INFINITY, f32::INFINITY);
            if child_value <= to_beat {
                to_beat = child_value;
                best_child = child;
                // println!("Best child is {:?}", best_child);
            } else {
                // println!("{} is not less than {}", child_value, to_beat);
            }
        }
        // let best_move = get_move_from_board_diff(board, &best_child).unwrap();
        // println!("best move is: {},{}", best_move.row, best_move.col);
        // println!("Score of best child was: {}", evaluate_board(&best_child));
        get_move_from_board_diff(board, &best_child)
    }
}

fn alphabeta(board: &OthelloPosition, depth: u32, mut alpha: f32, beta: f32) -> f32 {
    // println!("In alphabeta");
    if depth == 0 || is_game_over(board) {
        return evaluate_board(board);
    }
    let mut value: f32;
    if board.max_player {
        value = f32::NEG_INFINITY;
        for child in generate_children(board) {
            let child_value = alphabeta(&child, depth - 1, alpha, beta);
            if child_value > value {
                // value = max(value, child_value)
                value = child_value;
            }
            if value >= beta {
                break; // Beta cutoff
            }
            if value > alpha {
                // alpha = max(alpha, value)
                alpha = value;
            }
        }
        //println!("Returned value for white's turn: {}", value);
        return value;
    } else {
        value = f32::INFINITY;
        for child in generate_children(board) {
            let child_value = alphabeta(&child, depth - 1, alpha, beta);
            if child_value < value {
                value = child_value;
            }
            if value <= alpha {
                break; // Alpha cutoff
            }
        }
        //println!("Returned value for black's turn: {}", value);
        return value;
    }
}
