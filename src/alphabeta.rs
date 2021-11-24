/// This file implements the logic for generating a recommended
/// move, given a specific board. The algorithm used to do so is
/// minimax with alpha-beta pruning. Additionally, an attempt at
/// basic move-ordering is made along with the utilization of a
/// HashMap for storing board values.
use crate::board::OthelloPosition;
use crate::evaluator::Evaluator;
use crate::move_generator::{get_move_from_board_diff, Move};
use std::collections::HashMap;
use std::time::{Duration, Instant};
extern crate crossbeam;
pub const VERY_HIGH: isize = 9999999999999;
pub const VERY_LOW: isize = -VERY_HIGH;


/// Utilisies iterative deepening search in order to
/// inspect and evaluate the potential boards that can
/// be reached from the given board. Keeps searching until
/// either a set depth-limit is reached or the parsed time
/// limit is reached.
/// 
/// # Arguments
/// 
/// * `board` - The starting board used as the root of the search tree.
/// * `start_time` - The Instant represeting the start of program execution.
/// * `time_limit` - An integer representing the set time limit in secodns.
pub fn alphabeta_move_gen(
    board: &OthelloPosition,
    start_time: Instant,
    time_limit: u64,
) -> Option<Move> {
    let max_depth = 100000000;
    let mut depth_limit = 1;
    let mut best_move = None;
    let mut children = board.generate_children();
    let mut value_map = HashMap::new();
    while depth_limit <= max_depth
        && Instant::now().duration_since(start_time) < Duration::new(time_limit, 0)
    {
        best_move = alphabeta_at_root(
            board,
            &mut children,
            depth_limit,
            start_time,
            time_limit,
            &mut value_map,
        );
        depth_limit += 1;
        if board.max_player {
            children.sort_by(|a, b| b.score.cmp(&a.score)); // Descending
        } else {
            children.sort_by(|a, b| a.score.cmp(&b.score)); // Ascending
        }
    }

    best_move
}


/// Iterates over the children of the board representing
/// the root of the search tree used to find the best move.
/// From the best child, get_move_from_board_diff is called
/// in order to generate the recommended move.
/// 
/// # Arguments
/// 
/// * `board` - An OthelloPosition representing the board to start the search from.
/// * `children` - The children of the starting board. Passed to reduce computation.
/// * `depth_limit` - The maximum depth to search to.
/// * `start_time` - An Instant representing the time the program execution started.
/// * `time_limit` - An integer representing the maximum allowed search time in seconds.
/// * `value_map` - A HashMap containing String representations of boards and the evaluated values of the corresponding board.
/// 
pub fn alphabeta_at_root(
    board: &OthelloPosition,
    children: &mut Vec<OthelloPosition>,
    depth_limit: u32,
    start_time: Instant,
    time_limit: u64,
    value_map: &mut HashMap<String, isize>,
) -> Option<Move> {
    if children.len() == 0 {
        return None;
    }
    if board.max_player {
        let mut best_child = &OthelloPosition::worst_for_max(); //this should be replaced by any child
        let mut to_beat = VERY_LOW; // basically negative infinity as an integer
        for child in children {
            let child_value: isize;
            match value_map.get(&child.string_rep()) {
                Some(value) => child_value = *value,
                None => {
                    child_value = alphabeta(
                        child,
                        depth_limit,
                        VERY_LOW,
                        VERY_HIGH,
                        start_time,
                        time_limit,
                    );
                    value_map.insert(child.string_rep(), child_value);
                }
            }
            child.score = child_value;
            if child_value >= to_beat {
                to_beat = child_value;
                best_child = child;
            }
        }
        get_move_from_board_diff(board, &best_child)
    } else {
        let mut best_child = &OthelloPosition::worst_for_min();
        let mut to_beat = VERY_HIGH;
        for child in children {
            let child_value: isize;
            match value_map.get(&child.string_rep()) {
                Some(value) => child_value = *value,
                None => {
                    child_value = alphabeta(
                        child,
                        depth_limit,
                        VERY_LOW,
                        VERY_HIGH,
                        start_time,
                        time_limit,
                    );
                    value_map.insert(child.string_rep(), child_value);
                }
            }
            child.score = child_value;
            if child_value <= to_beat {
                to_beat = child_value;
                best_child = child;
            }
        }
        get_move_from_board_diff(board, &best_child)
    }
}

/// The actual minimax algorithm with alpha-beta pruning.
/// Evaluates the given board if the depth limit is reached, no moves are possible,
/// or if the time limit is reached.
/// 
/// # Arguments
/// 
/// * `board` - An OthelloPosition instance representing the board to be evaluated.
/// * `depth` - An integer representing the depth remaining until the search depth, decreases with each call.
/// * `alpha` - An integer representing the alpha parameter used for pruning.
/// * `beta` - An integer representing the beta parameter used for pruning.
/// * `start_time` - An Instant representing the time of program execution start.
/// * `time_limit` - An integer representing the maximum allowed search time in seconds.
pub fn alphabeta(
    board: &OthelloPosition,
    depth: u32,
    mut alpha: isize,
    mut beta: isize,
    start_time: Instant,
    time_limit: u64,
) -> isize {
    
    if depth == 0
        || board.is_game_over()
        || Instant::now().duration_since(start_time) > Duration::new(time_limit, 0)
    {
        let value = Evaluator::default().evaluate(board);
        return value;
    }

    if board.max_player {
        let mut value = VERY_LOW;
        for mut child in board.generate_children() {
            let child_value = alphabeta(&child, depth - 1, alpha, beta, start_time, time_limit);
            child.score = child_value;
            if child_value >= value {
                value = child_value;
            }
            if value >= beta {
                break;
            }
            if value > alpha {
                alpha = value;
            }
        }
        value
    } else {
        let mut value = VERY_HIGH;
        for mut child in board.generate_children() {
            let child_value = alphabeta(&child, depth - 1, alpha, beta, start_time, time_limit);
            child.score = child_value;
            if child_value <= value {
                value = child_value;
            }
            if value <= alpha {
                break;
            }
            if value < beta {
                beta = value;
            }
        }
        value
    }
}
