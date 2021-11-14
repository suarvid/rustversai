use crate::board::Board;
use crate::moves::Move;
use std::time;


pub fn

// TODO: figure out if this should allow negative values
// TODO: 
pub fn evaluate_board(board: &Board, player: char) -> f32 {
    let mut heuristic_value = 0.0;
    for elem in &board.cells {
        let player_occurrences = elem
            .iter()
            .map(|x| if *x == player { 1 } else { 0 })
            .collect::<Vec<u8>>();
        heuristic_value += player_occurrences.iter().sum::<u8>() as f32;
    }

    heuristic_value
}

fn generate_children(board: &Board, player: char) -> Vec<Board> {
    let possible_moves = Move::gen_valid_moves(board, player);
    let mut child_boards = Vec::new();

    for p_move in possible_moves {
        match Move::make_move(&board, &Some(p_move)) {
            Some(nb) => child_boards.push(nb),
            None => continue
        }
    }

    child_boards
}

// Maybe unnecessary to take a player, but think we always have access to one
pub fn is_game_over(board: &Board, player: char) -> bool {
    Move::gen_valid_moves(board, player).len() == 0
}

// Basic minimax without any pruning.
// Seems to work for depths ~10 for timing < 1 second, at least for initial tests
pub fn minimax(board: &Board, depth: u8, player: char) -> f32 {
    if depth == 0 || is_game_over(board, player) {
        return evaluate_board(board, player);
    }

    let children = generate_children(board, player);

    if player == 'w' {
        let mut value = f32::NEG_INFINITY;
        for child in children {
            let child_value = minimax(&child, depth - 1, 'b');
            if child_value > value {
                value = child_value;
            }
        }
        return value;
    } else {
        let mut value = f32::INFINITY;
        for child in children {
            let child_value = minimax(&child, depth - 1, 'w');
            if child_value < value {
                value = child_value;
            }
        }
        return value;
    }
}

fn get_move(from: &Board, to: &Board) -> Option<Move> {
    for row in (0..from.cells.len()) {
        let (differs, col, to_value) = vec_differs(&from.cells[row], &to.cells[row]);
        if differs {
            return Some(Move::new(to_value, row, col as usize));
        }
    }

    None
}

// Compares two vectors to see if they differ
fn vec_differs(fst: &Vec<char>, snd: &Vec<char>) -> (bool, isize, char) {
    for i in (0..fst.len()) {
        if fst[i] != snd[i] {
            return (true, i as isize, snd[i]);
        }
    }

    (false, -1, 'e')
}


pub fn alphabeta_move_gen(board: &Board, player: char) -> Option<Move> {
    let mut depth = 1;
    let start_time = time::Instant::now();
    let possible_moves = Move::gen_valid_moves(board, player);
    // max = 'w' wants to maximize, min = 'b' wants to minimize
    if player == 'w' {
        let mut best_val = f32::NEG_INFINITY;
        let mut best_child = Board::start_board(); // TODO: Make sure this placeholder val is okay, slightly ugly but whatever
        for candidate_move in possible_moves {
            match Move::make_move(board, &Some(candidate_move)) {
                Some(child) => {
                    let child_value = alphabeta(
                        board,
                       f32::NEG_INFINITY,
                        f32::INFINITY,
                        player,
                        start_time,
                        &mut depth
                    );
                    if child_value > best_val {
                        best_val = child_value;
                        best_child = child;
                    }
                }
                None => continue,
            }
        }
        println!("Maximum move depth reached: {}", depth);
        return get_move(&board, &best_child);
    } else {
        let mut best_val = f32::INFINITY;
        let mut best_child = Board::start_board();
        for candidate_move in possible_moves {
            match Move::make_move(board, &Some(candidate_move)) {
                Some(child) => {
                    let child_value = alphabeta(
                        board,
                       f32::NEG_INFINITY,
                        f32::INFINITY,
                        player,
                        start_time,
                        &mut depth
                    );
                    if child_value < best_val {
                        best_val = child_value;
                        best_child = child;
                    }
                }
                None => continue,
            }
        }
        println!("Number of nodes expanded: {}", depth); // Depth is not actually depth, just nb of nodes
        return get_move(&board, &best_child);
    }
}

// if i generate the children of the current board, call alphabeta on them, I can get the move that will be best, probably
pub fn alphabeta(board: &Board, mut alpha: f32, mut beta: f32, player: char, start_time: time::Instant, depth: &mut u8) -> f32 {
    if  is_game_over(board, player) || time::Instant::now().duration_since(start_time) > time::Duration::new(1, 0) {
        return evaluate_board(board, player);
    }

    let children = generate_children(board, player);
    *depth += 1;
    if player == 'w' {
        let mut value = f32::NEG_INFINITY;
        for child in children {
            let child_value = alphabeta(&child, alpha, beta, 'b', start_time, depth);
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
            let child_value = alphabeta(&child, alpha, beta, 'w', start_time, depth);
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
