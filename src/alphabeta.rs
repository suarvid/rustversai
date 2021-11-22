use crate::board::OthelloPosition;
use crate::evaluator::Evaluator;
use crate::move_generator::{get_move_from_board_diff, Move};
use std::collections::HashMap;
use std::time::{Duration, Instant};
extern crate crossbeam;
pub const VERY_HIGH: isize = 9999999999999;
pub const VERY_LOW: isize = -VERY_HIGH;
static mut NODES_EXPANDED: usize = 0;

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

pub fn alphabeta_at_root(
    board: &OthelloPosition,
    children: &mut Vec<OthelloPosition>,
    depth_limit: u32,
    start_time: Instant,
    time_limit: u64,
    value_map: &mut HashMap<String, isize>,
) -> Option<Move> {
    if board.max_player {
        let mut best_child = &OthelloPosition::worst_for_max(); //this should be replaced by any child
        let mut to_beat = VERY_LOW; // basically negative infinity as an integer
        if children.len() == 0 {
            return None;
        }
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
        if children.len() == 0 {
            return None;
        }
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

pub fn alphabeta(
    board: &OthelloPosition,
    depth: u32,
    mut alpha: isize,
    mut beta: isize,
    start_time: Instant,
    time_limit: u64,
) -> isize {
    unsafe {
        NODES_EXPANDED += 1;
    }
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
            if value >= alpha {
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
            if value <= beta {
                beta = value;
            }
        }
        value
    }
}
