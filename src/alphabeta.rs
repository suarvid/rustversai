use crate::move_generator::{get_move_from_board_diff, get_moves, Move, OthelloPosition};
use crate::move_generator::{EMPTY_CELL, PLAYER_BLACK, PLAYER_WHITE};
use std::time;
extern crate crossbeam;
pub const VERY_HIGH: isize = 9999999999999;
pub const VERY_LOW: isize = -VERY_HIGH;

// TODO: figure out if this should allow negative values
// It should probably return values in the range (-100, 100) for example
// Things that are important:
// Coin Parity
// Mobility
// Corners
// Stability

// Need to take into consideration potential mobility: number of opponent's disk next to an empty space
// Squares that will most likely give away the number of corners should have a highly negative weight
//      These squares are [2][2] [2][7] [7][2] and [7][7]

pub fn evaluate_board(board: &OthelloPosition) -> isize {
    if is_win_black(board) {
        return VERY_LOW;
    }
    if is_win_white(board) {
        return VERY_HIGH;
    }

    // Should potential_mobility > immediate_mobility, or vice versa?
     (1000 * corners_value(board))  + (200 * immediate_mobility(board)) + (20 * giving_away_corners(board)) + (100 * potential_mobility(board))
}

pub fn is_win_black(board: &OthelloPosition) -> bool {
    board.is_full() && board.num_black_pieces() > board.num_white_pieces()
}

pub fn is_win_white(board: &OthelloPosition) -> bool {
    board.is_full() && board.num_white_pieces() > board.num_black_pieces()
}

pub fn is_winning_board(board: &OthelloPosition) -> bool {
    if board.max_player {
        if board.is_full() && board.num_white_pieces() > board.num_black_pieces() {
            return true;
        }
    } else {
        if board.is_full() && board.num_black_pieces() > board.num_white_pieces() {
            return true;
        }
    }

    false
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

fn giving_away_corners(board: &OthelloPosition) -> isize {
    let mut value = 0;
    match board.board[2][2] {
        PLAYER_WHITE => value -= 1,
        PLAYER_BLACK => value += 1,
        _ => (),
    };
    match board.board[2][7] {
        PLAYER_WHITE => value -= 1,
        PLAYER_BLACK => value += 1,
        _ => (),
    };
    match board.board[7][2] {
        PLAYER_WHITE => value -= 1,
        PLAYER_BLACK => value += 1,
        _ => (),
    }
    match board.board[7][7] {
        PLAYER_WHITE => value -= 1,
        PLAYER_BLACK => value += 1,
        _ => (),
    }
    match board.board[2][1] {
        PLAYER_WHITE => value -= 1,
        PLAYER_BLACK => value += 1,
        _ => (),
    }
    match board.board[1][2] {
        PLAYER_WHITE => value -= 1,
        PLAYER_BLACK => value += 1,
        _ => (),
    }
    match board.board[1][7] {
        PLAYER_WHITE => value -= 1,
        PLAYER_BLACK => value += 1,
        _ => (),
    }
    match board.board[2][8] {
        PLAYER_WHITE => value -= 1,
        PLAYER_BLACK => value += 1,
        _ => (),
    }
    match board.board[7][1] {
        PLAYER_WHITE => value -= 1,
        PLAYER_BLACK => value += 1,
        _ => (),
    }
    match board.board[7][8] {
        PLAYER_WHITE => value -= 1,
        PLAYER_BLACK => value += 1,
        _ => (),
    }
    match board.board[8][2] {
        PLAYER_WHITE => value -= 1,
        PLAYER_BLACK => value += 1,
        _ => (),
    }
    match board.board[8][7] {
        PLAYER_WHITE => value -= 1,
        PLAYER_BLACK => value += 1,
        _ => (),
    }
    value
}
// This is a behemoth of a function
// TODO: Extract this into smaller functions
fn potential_mobility(board: &OthelloPosition) -> isize {
    let mut white_count = 0;
    let mut black_count = 0;
        for row in 1..=8 {
            for col in 1..=8 {
                if board.board[row][col] == PLAYER_BLACK {
                    let above = board.board[row - 1][col];
                    let below = board.board[row - 1][col];
                    let right = board.board[row][col + 1];
                    let left = board.board[row][col - 1];
                    let top_right = board.board[row - 1][col + 1];
                    let top_left = board.board[row - 1][col - 1];
                    let bot_right = board.board[row + 1][col + 1];
                    let bot_left = board.board[row + 1][col - 1];
                    if row == 1 { // Can't look up
                        if col == 1 { // Can't look left
                            if right == EMPTY_CELL { white_count+= 1};
                            if below == EMPTY_CELL { white_count += 1};
                            if bot_right == EMPTY_CELL { white_count += 1};
                        } else if col == 8 { // Can't look right
                            if left == EMPTY_CELL {white_count += 1};
                            if below == EMPTY_CELL {white_count += 1};
                            if bot_left == EMPTY_CELL {white_count += 1};
                        } else {
                            if left == EMPTY_CELL {white_count += 1};
                            if right == EMPTY_CELL {white_count += 1};
                            if bot_left == EMPTY_CELL {white_count += 1};
                            if below == EMPTY_CELL {white_count += 1};
                            if bot_right == EMPTY_CELL {white_count += 1};
                        }
                    } else if row == 8 { // Can't look down
                        if col == 1 { // Can't look left
                            if above == EMPTY_CELL {white_count += 1};
                            if top_right == EMPTY_CELL {white_count += 1};
                            if right == EMPTY_CELL {white_count += 1};
                        } else if col == 8 { // Can't look right
                            if above == EMPTY_CELL {white_count += 1};
                            if top_left == EMPTY_CELL {white_count += 1};
                            if left == EMPTY_CELL {white_count += 1};
                        } else {
                            if top_left == EMPTY_CELL {white_count += 1};
                            if top_right == EMPTY_CELL {white_count += 1};
                            if left == EMPTY_CELL {white_count += 1};
                            if right == EMPTY_CELL {white_count += 1};
                        }
                    } else { // Can look both up and down
                        if above == EMPTY_CELL {white_count += 1};
                        if below == EMPTY_CELL {white_count += 1};
                        if col == 1 { // Cannot look left
                            if top_right == EMPTY_CELL {white_count += 1};
                            if right == EMPTY_CELL {white_count += 1};
                            if bot_right == EMPTY_CELL {white_count += 1};
                        } else if col == 8 { // Cannot look right
                            if top_left == EMPTY_CELL {white_count+= 1};
                            if left == EMPTY_CELL {white_count+= 1};
                            if bot_left == EMPTY_CELL {white_count+= 1};
                        }
                    }
                } else if board.board[row][col] == PLAYER_WHITE {
                    let above = board.board[row - 1][col];
                    let below = board.board[row - 1][col];
                    let right = board.board[row][col + 1];
                    let left = board.board[row][col - 1];
                    let top_right = board.board[row - 1][col + 1];
                    let top_left = board.board[row - 1][col - 1];
                    let bot_right = board.board[row + 1][col + 1];
                    let bot_left = board.board[row + 1][col - 1];
                    if row == 1 { // Can't look up
                        if col == 1 { // Can't look left
                            if right == EMPTY_CELL { black_count += 1};
                            if below == EMPTY_CELL { black_count += 1};
                            if bot_right == EMPTY_CELL { black_count += 1};
                        } else if col == 8 { // Can't look right
                            if left == EMPTY_CELL {black_count += 1};
                            if below == EMPTY_CELL {black_count += 1};
                            if bot_left == EMPTY_CELL {black_count += 1};
                        } else {
                            if left == EMPTY_CELL {black_count += 1};
                            if right == EMPTY_CELL {black_count += 1};
                            if bot_left == EMPTY_CELL {black_count += 1};
                            if below == EMPTY_CELL {black_count += 1};
                            if bot_right == EMPTY_CELL {black_count += 1};
                        }
                    } else if row == 8 { // Can't look down
                        if col == 1 { // Can't look left
                            if above == EMPTY_CELL {black_count += 1};
                            if top_right == EMPTY_CELL {black_count += 1};
                            if right == EMPTY_CELL {black_count += 1};
                        } else if col == 8 { // Can't look right
                            if above == EMPTY_CELL {black_count += 1};
                            if top_left == EMPTY_CELL {black_count += 1};
                            if left == EMPTY_CELL {black_count += 1};
                        } else {
                            if top_left == EMPTY_CELL {black_count += 1};
                            if top_right == EMPTY_CELL {black_count += 1};
                            if left == EMPTY_CELL {black_count += 1};
                            if right == EMPTY_CELL {black_count += 1};
                        }
                    } else { // Can look both up and down
                        if above == EMPTY_CELL {black_count += 1};
                        if below == EMPTY_CELL {black_count += 1};
                        if col == 1 { // Cannot look left
                            if top_right == EMPTY_CELL {black_count += 1};
                            if right == EMPTY_CELL {black_count += 1};
                            if bot_right == EMPTY_CELL {black_count += 1};
                        } else if col == 8 { // Cannot look right
                            if top_left == EMPTY_CELL {black_count += 1};
                            if left == EMPTY_CELL {black_count += 1};
                            if bot_left == EMPTY_CELL {black_count += 1};
                        }
                    }
                }
            }
        }

    white_count - black_count
}

fn immediate_mobility(board: &OthelloPosition) -> isize {
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
        child_boards.push(Move::make_move(&board, &p_move));
    }

    child_boards
}

// Maybe unnecessary to take a player, but think we always have access to one
// TODO: Rewrite this to recognize that the game is over
pub fn is_game_over(board: &OthelloPosition) -> bool {
    for row in 1..=8 {
        for col in 1..=8 {
            if board.board[row][col] == 'E' {
                // at least one empty space remaining
                return false;
            }
        }
    }

    true
}

pub fn alphabeta_move_gen(
    board: &OthelloPosition,
    start_time: time::Instant,
    time_limit: u64,
) -> Option<Move> {
    let max_depth = 110;
    let mut depth_limit = 1;
    let mut best_move = None;
    let time_duration = time::Duration::new(time_limit, 0);
    while depth_limit <= max_depth
        && time::Instant::now().duration_since(start_time) < time_duration //TODO: Remove this when running for real
    {
        best_move = alphabeta_at_root(board, depth_limit);
        depth_limit += 1;
    }
    //println!("Max depth reached was: {}", depth_limit);

    best_move
}

// 4 unique children are generated, given the starting string
// Values are always negative when playing as white, always positive when playing as black
// Though that might be because the other player gets to move first when considering child nodes (sort of)
pub fn alphabeta_at_root(board: &OthelloPosition, depth_limit: u32) -> Option<Move> {
    // max wants child with max value,
    // min wants child with min value
    let children = generate_children(board);
    if board.max_player {
        crossbeam::scope(|s| {
            let mut best_child = &OthelloPosition::worst_for_max(); //this should be replaced by any child
            let mut to_beat = VERY_LOW; // basically negative infinity as an integer
            if children.len() == 0 {
                return None;
            }
            for child in &children {
                let thread = s.spawn(move |_| alphabeta(child, depth_limit, VERY_LOW, VERY_HIGH));
                let child_value = thread.join().unwrap(); // When debugging: every child has exactly the same value each pass
        //        let child_value = alphabeta(child, depth_limit, VERY_LOW, VERY_HIGH);
                if child_value >= to_beat {
                    to_beat = child_value;
                    best_child = child;
                }
            }

            get_move_from_board_diff(board, &best_child)
        })
        .unwrap()
    } else {
        crossbeam::scope(|s| {
            let mut best_child = &OthelloPosition::worst_for_min();
            let mut to_beat = VERY_HIGH;
            if children.len() == 0 {
                return None;
            }
            for child in &children {
                let thread = s.spawn(move |_| alphabeta(child, depth_limit, VERY_LOW, VERY_HIGH));
                let child_value = thread.join().unwrap();
         //       let child_value = alphabeta(child, depth_limit, VERY_LOW, VERY_HIGH);
                if child_value <= to_beat {
                    to_beat = child_value;
                    best_child = child;
                }
            }
            // println!("Best value found for black is: {}", evaluate_board(best_child));
            get_move_from_board_diff(board, &best_child)
        })
        .unwrap()
    }
}

pub fn alphabeta(board: &OthelloPosition, depth: u32, mut alpha: isize, mut beta: isize) -> isize {

    if depth == 0 || is_game_over(board) {
        let value = evaluate_board(board);
        return value;
    }

    if board.max_player {
        // println!("WHITE ALPHABETA");
        let mut value = VERY_LOW;
        for child in generate_children(board) {
            let child_value = alphabeta(&child, depth - 1, alpha, beta);
            if child_value >= value {
                value = child_value;
            }
            if value >= beta {
                continue; //Does this still count as pruning? We don't win if we call break instead
            }
            if value >= alpha {
                alpha = value;
            }
        }
        value
    } else {
        // For every call to white alpha beta, there seems to be 3-4 calls to black alpha beta
        // println!("BLACK ALPHABETA");
        let mut value = VERY_HIGH;
        for child in generate_children(board) {
            let child_value = alphabeta(&child, depth - 1, alpha, beta);
            if child_value <= value {
                value = child_value;
            }
            if value <= alpha {
                continue; //If we call break, we lose
            }
            if value <= beta {
                beta = value;
            }
        }
        value
    }
}
