use crate::board;
use crate::board::Board;
use crate::board::EMPTY_CELL;
use std::fmt;
use std::iter;

#[derive(Debug, Clone)]
pub struct Move {
  player: char,
  row: usize,
  col: usize,
}

impl Move {
  pub fn new(player: char, row: usize, col: usize) -> Move {
    Move { player, row, col }
  }

  pub fn make_move<'a>(board: &Board, to_make: &Option<Move>) -> Option<Board> {
    match to_make {
      Some(m) => Some(board.add_piece(m.row, m.col, m.player)),
      None => None,
    }
  }

  // TODO: This is sound, but not complete!
  pub fn gen_valid_moves(board: &Board, player: char) -> Vec<Move> {
    let mut row_moves = gen_valid_moves_helper(&board, player);
    let mut col_moves = gen_valid_moves_helper(&board.transpose(), player);
    //TODO: do the same thing but for diagonals!
    for mut transposed_move in &mut col_moves {
      let temp = transposed_move.row;
      transposed_move.row = transposed_move.col;
      transposed_move.col = temp;
    }
    row_moves.append(&mut col_moves);
    row_moves.append(&mut gen_moves_diagonals(&board, player));
    row_moves
  }
}

impl fmt::Display for Move {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "player: {}, row: {}, column: {}",
      self.player, self.row, self.col
    )
  }
}

pub fn get_move_from_board_diff(from: &Board, to: &Board) -> Option<Move> {
  for row in 0..from.cells.len() {
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

  (false, -1, EMPTY_CELL)
}

// TODO: Move some stuff into a while-loop to find all moves instead of just some, which is the case now
fn gen_valid_moves_helper(board: &Board, player: char) -> Vec<Move> {
  let mut row_number = 0;
  let mut valid_moves = Vec::new();
  for row in &board.cells {
    let row_positions = valid_pos_in_row(row.to_vec(), player);
    for position in row_positions {
      let (row, col) = (row_number, position);
      let valid_move = Move::new(player, row, col);
      valid_moves.push(valid_move);
    }
    row_number += 1;
  }

  valid_moves
}

fn gen_moves_diagonals(board: &Board, player: char) -> std::vec::Vec<Move> {
  let mut valid_moves = Vec::new();

  for i in (0..=7).rev() {
    let left_to_right_diag = board::get_diagonal_left_to_right(board, i);
    valid_moves.push(gen_valid_moves_diagonal(
      left_to_right_diag,
      true,
      7,
      i as usize,
      player,
    ));
  }

  for j in (0..=7) {
    let right_to_left_diag = board::get_diagonal_right_to_left(board, j);
    valid_moves.push(gen_valid_moves_diagonal(
      right_to_left_diag,
      false,
      7,
      j as usize,
      player,
    ));
  }

  valid_moves.concat()
}

// diagonals above the main diagonal make it more complicated
// generates the valid moves in a given diagonal
fn gen_valid_moves_diagonal(
  diagonal: Vec<char>,
  left_to_right: bool,
  lowest_row: usize,
  edge_column: usize,
  player: char,
) -> std::vec::Vec<Move> {
  let to_transform = valid_pos_in_row(diagonal, player);
  let mut valid_diag_moves = Vec::new();

  // if left-to-right, the index will represent the column
  // the question is which row it belongs in
  // index of lowest row - (edge_column - col_num) maybe?
  if left_to_right {
    for diag_position in to_transform {
      let col_num = diag_position;
      let row_num = lowest_row - (edge_column - col_num);
      valid_diag_moves.push(Move::new(player, row_num, col_num));
    }
  } else {
    // diag is right-to-left
    // column index should still be correct
    //
    for diag_position in to_transform {
      let col_num = diag_position;
      let row_num = lowest_row - (col_num - edge_column);
      valid_diag_moves.push(Move::new(player, row_num, col_num));
    }
  }

  valid_diag_moves
}

// valid places to place pieces are empty indexes that have first one colour
// then the other, without any empty slots between them, on either side of the empty slot
// TODO: Make sure this works
pub fn valid_pos_in_row(row: Vec<char>, player: char) -> std::vec::Vec<usize> {
  let mut legal_positions = Vec::new();

  let mut current_index = 0;
  // first, skip any initial empty slots in the row
  while current_index < row.len() && row[current_index] == EMPTY_CELL {
    current_index += 1;

    if current_index == row.len() {
      return legal_positions; // no valid moves found in row
    }

    if row[current_index] != player && current_index > 0 {
      // opponent piece found as first non-empty, check that there is a player piece to its right
      let possible_placement = current_index - 1; // Save the empty position as possible return value // Subtract with overflow here

      // keep going as long as there are opponent pieces
      while current_index < row.len() - 1
        && row[current_index] != player
        && row[current_index] != EMPTY_CELL
      {
        current_index += 1;
      }

      if row[current_index] == player {
        //the initial index can be saved
        legal_positions.push(possible_placement);
      }
    } else {
      // players own piece found first
      while current_index < row.len() - 1 && row[current_index] == player {
        current_index += 1;
      }

      // first opponent piece found
      if current_index < 8 && row[current_index] != EMPTY_CELL {
        while row[current_index] != EMPTY_CELL && row[current_index] != player {
          current_index += 1;
        }
        if row[current_index] == EMPTY_CELL {
          legal_positions.push(current_index); // Found empty after leading player piece and consecutive opponent pieces
        }
      }
    }
  }

  legal_positions
}
