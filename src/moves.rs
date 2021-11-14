use crate::board::Board;
use std::fmt;
use std::iter;

#[derive(Debug)]
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
      None => None
    }
  }

  // TODO: This is sound, but not complete!
  pub fn gen_valid_moves(board: &Board, player: char) -> Vec<Move> {
    let mut row_moves = gen_valid_moves_helper(&board, player);
    let mut col_moves = gen_valid_moves_helper(&board.transpose(), player); //TODO: have to un-transpose the returned coordinates
    for mut transposed_move in &mut col_moves {
      let temp = transposed_move.row;
      transposed_move.row = transposed_move.col;
      transposed_move.col = temp;
    }
    row_moves.append(&mut col_moves);

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

// valid places to place pieces are empty indexes that have first one colour
// then the other, without any empty slots between them, on either side of the empty slot

pub fn valid_pos_in_row(row: Vec<char>, player: char) -> std::vec::Vec<usize> {
  let mut legal_positions = Vec::new();

  let mut current_index = 0;
  // first, skip any initial empty slots in the row
  while current_index < row.len() && row[current_index] == 'e' {
    current_index += 1;
  }

  if current_index == row.len() {
    return Vec::new(); // no valid moves found in row
  }

  // TODO:
  // Should probably move this into the while()-loop (or as own funciton) in order to check all possibilities.
  if row[current_index] != player && current_index > 0{
    // opponent piece found as first non-empty, check that there is a player piece to its right
    let possible_placement = current_index - 1; // Save the empty position as possible return value // Subtract with overflow here

    // keep going as long as there are opponent pieces
    while row[current_index] != player && row[current_index] != 'e' {
      current_index += 1;
    }

    if row[current_index] == player {
      //the initial index can be saved
      legal_positions.push(possible_placement);
    }
  } else {
    // players own piece found first
    while row[current_index] == player {
      current_index += 1;
    }

    // first opponent piece found
    if row[current_index] != 'e' {
      while row[current_index] != 'e' && row[current_index] != player {
        current_index += 1;
      }
      if row[current_index] == 'e' {
        legal_positions.push(current_index); // Found empty after leading player piece and consecutive opponent pieces
      }
    }
  }

  legal_positions
}
