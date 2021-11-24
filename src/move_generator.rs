use crate::board::{OthelloPosition, BOARD_SIZE, EMPTY_CELL, PLAYER_BLACK, PLAYER_WHITE};

#[derive(Debug, Clone, PartialEq)]
/// This file contains the implementation logic of a Move.
/// Additionally, it contains a rewrite of the move generation code
/// in the file OthelloPosition.java, given in the assignment.
/// It should therefore be noted that I should probably not be credited
/// as the author of the implementation of the logic in get_moves and the 
/// functions it relies on, although these have been slightly improved
/// from the given versions.

/// A representation of a Move.
pub struct Move {
    /// A char representing the colour of the player to add a piece.
    player: char,
    /// The row where the piece should be added.
    pub row: usize,
    /// The column where the piece should be added.
    pub col: usize,
}

impl Move {
    /// Returns a new instance of Move with the given player, row, and column.
    /// 
    /// # Arguments
    /// 
    /// * `player` - a char representation of the colour of the player to add a piece.
    /// * `row` - the row where the piece should be added.
    /// * `col` - the column where the piece should be added.
    pub fn new(player: char, row: usize, col: usize) -> Move {
        Move { player, row, col }
    }

    /// Takes a board and a move, adds the piece to the given board in the specified
    /// location and returns the resulting board as an OthelloPosition.
    /// 
    /// # Arguments
    /// 
    /// * `board` - The board to add a piece to.
    /// * `to_make` - A Move to be carried out on the given board.
    pub fn make_move<'a>(board: &OthelloPosition, to_make: &Move) -> OthelloPosition {
        board.add_piece(to_make.row + 1, to_make.col + 1, to_make.player)
    }
}

/// This function and the function it utilizies are heavily based on their
/// Java-based implementation given in the assignment files.
pub fn get_moves(board: &OthelloPosition) -> std::vec::Vec<Move> {
    let mut val_moves = Vec::new();
    let mut candidates = [[false; BOARD_SIZE + 2]; BOARD_SIZE + 2];
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            candidates[i][j] = is_candidate(board, i + 1, j + 1);
        }
    }
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if candidates[row][col] {
                if is_move(board, row + 1, col + 1) {
                    match board.max_player {
                        true => val_moves.push(Move::new(PLAYER_WHITE, row, col)),
                        false => val_moves.push(Move::new(PLAYER_BLACK, row, col)),
                    }
                }
            }
        }
    }

    val_moves
}

fn is_move(board: &OthelloPosition, row: usize, col: usize) -> bool {
    check_north(board, row, col)
        || check_north_east(board, row, col)
        || check_east(board, row, col)
        || check_south_east(board, row, col)
        || check_south(board, row, col)
        || check_south_west(board, row, col)
        || check_north_west(board, row, col)
        || check_west(board, row, col)
}

fn check_north(board: &OthelloPosition, row: usize, col: usize) -> bool {
    if !is_opponent_square(board, row - 1, col) {
        return false;
    }
    let mut i: isize = row as isize - 2;
    while i > 0 {
        if is_free(board, i as usize, col) {
            return false;
        }
        if is_own_square(board, i as usize, col) {
            return true;
        }
        i -= 1;
    }
    false
}

fn check_east(board: &OthelloPosition, row: usize, col: usize) -> bool {
    if !is_opponent_square(board, row, col + 1) {
        return false;
    }

    let mut i = col + 2;
    while i <= BOARD_SIZE {
        if is_free(board, row, i) {
            return false;
        }
        if is_own_square(board, row, i) {
            return true;
        }
        i += 1;
    }

    false
}

fn check_south(board: &OthelloPosition, row: usize, col: usize) -> bool {
    if !is_opponent_square(board, row + 1, col) {
        return false;
    }

    let mut i = row + 2;
    while i <= BOARD_SIZE {
        if is_free(board, i, col) {
            return false;
        }
        if is_own_square(board, i, col) {
            return true;
        }

        i += 1;
    }

    false
}

fn check_west(board: &OthelloPosition, row: usize, col: usize) -> bool {
    if !is_opponent_square(board, row, col - 1) {
        return false;
    }

    let mut i: isize = col as isize - 2;
    while i > 0 {
        if is_free(board, row, i as usize) {
            return false;
        }
        if is_own_square(board, row, i as usize) {
            return true;
        }
        i -= 1;
    }

    false
}

fn check_north_east(board: &OthelloPosition, row: usize, col: usize) -> bool {
    if !is_opponent_square(board, row - 1, col + 1) {
        return false;
    }
    let mut i: isize = 2;
    while (row as isize - i) as isize > 0 && (col + i as usize) <= BOARD_SIZE {
        if is_free(board, row - i as usize, col + i as usize) {
            return false;
        }
        if is_own_square(board, row - i as usize, col + i as usize) {
            return true;
        }

        i += 1;
    }

    false
}

fn check_south_east(board: &OthelloPosition, row: usize, col: usize) -> bool {
    if !is_opponent_square(board, row + 1, col + 1) {
        return false;
    }
    let mut i = 2;
    while (row + i) <= BOARD_SIZE && (col + i) <= BOARD_SIZE {
        if is_free(board, row + i, col + i) {
            return false;
        }
        if is_own_square(board, row + i, col + i) {
            return true;
        }
        i += 1;
    }

    false
}

fn check_south_west(board: &OthelloPosition, row: usize, col: usize) -> bool {
    if !is_opponent_square(board, row + 1, col - 1) {
        return false;
    }
    let mut i: isize = 2;
    while (row + i as usize) <= BOARD_SIZE && (col as isize - i) > 0 {
        if is_free(board, row + i as usize, col - i as usize) {
            return false;
        }
        if is_own_square(board, row + i as usize, col - i as usize) {
            return true;
        }
        i += 1;
    }

    false
}

fn check_north_west(board: &OthelloPosition, row: usize, col: usize) -> bool {
    if !is_opponent_square(board, row - 1, col - 1) {
        return false;
    }
    let mut i: isize = 2;
    while (row as isize - i) > 0 && (col as isize - i) > 0 {
        if is_free(board, row - i as usize, col - i as usize) {
            return false;
        }
        if is_own_square(board, row - i as usize, col - i as usize) {
            return true;
        }
        i += 1;
    }

    false
}

fn is_opponent_square(board: &OthelloPosition, row: usize, col: usize) -> bool {
    if board.max_player && board.board[row][col] == PLAYER_BLACK {
        return true;
    }
    if (!board.max_player) && board.board[row][col] == PLAYER_WHITE {
        return true;
    }

    false
}

fn is_own_square(board: &OthelloPosition, row: usize, col: usize) -> bool {
    if (!board.max_player) && board.board[row][col] == PLAYER_BLACK {
        return true;
    }
    if board.max_player && board.board[row][col] == PLAYER_WHITE {
        return true;
    }

    false
}

fn is_candidate(board: &OthelloPosition, row: usize, col: usize) -> bool {
    if !is_free(board, row, col) {
        return false;
    }
    if has_neighbor(board, row, col) {
        return true;
    }

    false
}

fn has_neighbor(board: &OthelloPosition, row: usize, col: usize) -> bool {
    !is_free(board, row - 1, col)
        || !is_free(board, row - 1, col + 1)
        || !is_free(board, row, col + 1)
        || !is_free(board, row + 1, col + 1)
        || !is_free(board, row + 1, col)
        || !is_free(board, row + 1, col - 1)
        || !is_free(board, row, col - 1)
        || !is_free(board, row - 1, col - 1)
}

fn is_free(board: &OthelloPosition, row: usize, col: usize) -> bool {
    board.board[row][col] == EMPTY_CELL
}


pub fn get_move_from_board_diff(from: &OthelloPosition, to: &OthelloPosition) -> Option<Move> {
    for row in 0..from.board.len() {
        let (differs, col, to_value) = vec_differs(&from.board[row], &to.board[row]);
        if differs {
            return Some(Move::new(to_value, row, col as usize));
        }
    }
    None
}


fn vec_differs(fst: &[char], snd: &[char]) -> (bool, isize, char) {
    for i in 0..fst.len() {
        if fst[i] != snd[i] {
            return (true, i as isize, snd[i]);
        }
    }

    (false, -1, EMPTY_CELL)
}
