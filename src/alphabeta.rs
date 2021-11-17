use crate::move_generator::{get_moves, get_move_from_board_diff, Move, OthelloPosition};
use crate::move_generator::{EMPTY_CELL, PLAYER_BLACK, PLAYER_WHITE};
use std::time;

static mut nodes_expanded: u32 = 0;

// TODO: figure out if this should allow negative values
pub fn evaluate_board(board: &OthelloPosition) -> f32 {
    let mut heuristic_value = 0.0;
    let mut player = 'X';
    if board.max_player {
        player = 'O';
    }

    for elem in &board.board {
        let player_occurrences = elem
            .iter()
            .map(|x| if *x == player { 1 } else { 0 })
            .collect::<Vec<u8>>();
        heuristic_value += player_occurrences.iter().sum::<u8>() as f32;
    }

    heuristic_value
}

fn generate_children(board: &OthelloPosition) -> Vec<OthelloPosition> {
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
pub fn is_game_over(board: &OthelloPosition) -> bool {
    unsafe {get_moves(board).len() == 0 || nodes_expanded > 100000} //TODO: Figure out if this is OK or if we need to avoid stack overflow some other way
}

pub fn alphabeta_move_gen(board: &OthelloPosition, time_limit: u64) -> Option<Move> {
    let start_time = time::Instant::now();
    let possible_moves = get_moves(board);
    
    // max = 'w' wants to maximize, min = 'b' wants to minimize
    if board.max_player {
        let mut best_val = f32::NEG_INFINITY;
        let mut best_child = OthelloPosition::empty(); // TODO: Make sure this placeholder val is okay, slightly ugly but whatever
        for candidate_move in possible_moves {
            match Move::make_move(board, &Some(candidate_move)) {
                Some(child) => {
                    let child_value =
                        alphabeta(board, f32::NEG_INFINITY, f32::INFINITY, start_time, time_limit);
                    if child_value > best_val {
                        best_val = child_value;
                        best_child = child;
                    }
                }
                None => panic!("No board found as result of making candidate move!"),
            }
        }
        return get_move_from_board_diff(&board, &best_child);
    } else {
        let mut best_val = f32::INFINITY;
        let mut best_child = OthelloPosition::empty();
        for candidate_move in possible_moves {
            match Move::make_move(board, &Some(candidate_move)) {
                Some(child) => {
                    let child_value =
                        alphabeta(board, f32::NEG_INFINITY, f32::INFINITY, start_time, time_limit);
                    if child_value < best_val {
                        best_val = child_value;
                        best_child = child;
                    }
                }
                None => panic!("No board found as result of making candidate move!"),
            }
        }
        return get_move_from_board_diff(&board, &best_child);
    }
}

// if i generate the children of the current board, call alphabeta on them, I can get the move that will be best, probably
fn alphabeta(
    board: &OthelloPosition,
    mut alpha: f32,
    mut beta: f32,
    start_time: time::Instant,
    time_limit: u64
) -> f32 {
    if is_game_over(board)
        || time::Instant::now().duration_since(start_time) > time::Duration::new(time_limit, 0)
    {
        return evaluate_board(board);
    }

    let children = generate_children(board);
    if board.max_player {
        let mut value = f32::NEG_INFINITY;
        for child in children {
            unsafe {
                nodes_expanded += 1;
            }
            let child_value = alphabeta(&child, alpha, beta, start_time, time_limit);
            if child_value > value {
                value = child_value;
            }
            if value >= beta {
                break; // beta cutoff
            } else if value > alpha {
                alpha = value;
            }
        }
        return value;
    } else {
        let mut value = f32::INFINITY;
        for child in children {
            unsafe {
                nodes_expanded += 1;
            }
            let child_value = alphabeta(&child, alpha, beta, start_time, time_limit);
            if child_value < value {
                value = child_value;
            }
            if value <= alpha {
                break;
            } else if value < beta {
                beta = value;
            }
        }
        return value;
    }
}
