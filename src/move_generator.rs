pub const PLAYER_WHITE: char = 'O';
pub const PLAYER_BLACK: char = 'X';
pub const EMPTY_CELL: char = 'E';
pub const BOARD_SIZE: usize = 8;

#[derive(Debug, Clone)]
pub struct Move {
  player: char,
  pub row: usize,
  pub col: usize,
}

#[derive(Debug)]
pub struct OthelloPosition {
    pub board: Vec<Vec<char>>, // If we don't use Vectors, we get stack overflow lol
    pub max_player: bool
}

impl OthelloPosition {
    pub fn empty() -> OthelloPosition{
        let board = vec![vec![EMPTY_CELL; 10]; 10];

        OthelloPosition { board, max_player: true }
    }

    pub fn new(string_rep: &str) -> OthelloPosition{
        if string_rep.len() != 65 {
            OthelloPosition::empty()
        } else {
            let mut board = vec![vec![EMPTY_CELL; 10]; 10];
            let mut max_player = false;
            if string_rep.chars().collect::<Vec<char>>()[0] == PLAYER_WHITE {
                max_player = true;
            }
            for i in 1..=64 {
                let c = string_rep.chars().collect::<Vec<char>>()[i];
                let col = ((i - 1) % BOARD_SIZE) + 1;
                let row = (i - 1) / 8 + 1;
                board[row][col] = c;
            }
            OthelloPosition{board, max_player}
        }
    }

    pub fn add_piece(&self, row: usize, col: usize, player: char) -> OthelloPosition {
        let mut new_position = OthelloPosition{board: self.board.clone(), max_player: !self.max_player};
        new_position.board[row][col] = player;
        new_position
    }
}


impl Move {
  pub fn new(player: char, row: usize, col: usize) -> Move {
    Move { player, row, col }
  }

  pub fn make_move<'a>(board: &OthelloPosition, to_make: &Option<Move>) -> Option<OthelloPosition> {
    match to_make {
      Some(m) => Some(board.add_piece(m.row, m.col, m.player)),
      None => None,
    }
  }
}


pub fn getMoves(board: &OthelloPosition) -> std::vec::Vec<Move> {
    let mut val_moves = Vec::new();
    let mut candidates = [[false; BOARD_SIZE + 2]; BOARD_SIZE + 2];
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            candidates[i][j] = isCandidate(board, i + 1, j + 1);
        }
    }
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if candidates[row][col] {
                if isMove(board, row + 1, col + 1,) {
                    match board.max_player {
                        true => val_moves.push(Move::new(PLAYER_WHITE, row, col )),
                        false => val_moves.push(Move::new(PLAYER_BLACK, row, col)),
                    }
                }
            }
        }
    }

    val_moves
}

fn isMove(board: &OthelloPosition, row: usize, col: usize) -> bool {
    checkNorth(board, row, col) || checkNorthEast(board, row, col) || checkEast(board, row, col) || checkSouthEast(board, row, col) || checkSouth(board, row, col) || checkSouthWest(board, row, col) || checkNorthWest(board, row, col)
}

fn checkNorth(board: &OthelloPosition, row: usize, col: usize) -> bool {
    if !isOpponentSquare(board, row - 1, col) {
        return false;
    }
    for i in (0..=row-2).rev() {
        if isFree(board, i, col) {
            return false;
        }
        if isOwnSquare(board, i, col) {
            return true;
        }
    }

    false
}

fn checkEast(board: &OthelloPosition, row: usize, col: usize) -> bool {
    if !isOpponentSquare(board, row, col + 1) {
        return false;
    }
    for i in (col+2)..BOARD_SIZE {
        if isFree(board, row, i) {
            return false;
        }
        if isOwnSquare(board, row, i) {
            return true;
        }
    }

    false
}

fn checkSouth(board: &OthelloPosition, row: usize, col: usize) -> bool {
    if !isOpponentSquare(board, row + 1, col) {
        return false;
    }
    for i in (row + 2)..=BOARD_SIZE {
        if isFree(board, i, col) {
            return false;
        }
        if isOwnSquare(board, i, col) {
            return true;
        }
    }

    false
}

fn checkWest(board: &OthelloPosition, row: usize, col: usize) -> bool {
    if !isOpponentSquare(board, row, col - 1) {
        return false;
    }

    for i in (0..=(col - 2)).rev() {
        if isFree(board, row, i) {
            return false;
        }
        if isOwnSquare(board, row, i) {
            return true;
        }
    }

    false
}

fn checkNorthEast(board: &OthelloPosition, row: usize, col: usize) -> bool {
    if !isOpponentSquare(board, row - 1, col + 1) {
        return false;
    }
    let mut i = 2;
    while (row - i) > 0 && (col + i) <= BOARD_SIZE {
        if isFree(board, row - i, col + i) {
            return false;
        }
        if isOwnSquare(board, row - i, col + i) {
            return true;
        }

        i += 1;
    }

    false
}

fn checkSouthEast(board: &OthelloPosition, row: usize, col: usize) -> bool {
    if !isOpponentSquare(board, row + 1, col + 1) {
        return false;
    }
    let mut i = 2;
    while (row + i) <= BOARD_SIZE && (col + i) <= BOARD_SIZE {
        if isFree(board, row + i, col + i) {
            return false;
        }
        if isOwnSquare(board, row + i, col + i) {
            return true;
        }
        i += 1;
    }

    false
}

fn checkSouthWest(board: &OthelloPosition, row: usize, col: usize) -> bool {
    if !isOpponentSquare(board, row + 1, col - 1) {
        return false;
    }
    let mut i = 2;
    while (row + i) <= BOARD_SIZE && (col - i) > 0 {
        if isFree(board, row + i, col - i) {
            return false;
        }
        if isOwnSquare(board, row + i, col - i) {
            return true;
        }
        i += 1;
    }

    false
}

fn checkNorthWest(board: &OthelloPosition, row: usize, col: usize) -> bool {
  if !isOpponentSquare(board, row - 1, col - 1) {
    return false;
  }
  let mut i = 2;
  while (row - 1) > 0 && (col - 1 > 0) {
    if isFree(board, row - i, col - i) {
        return false;
    }
    if isOwnSquare(board, row - i, col - i) {
        return true
    }
    i += 1;
  }

  false

}


fn isOpponentSquare(board: &OthelloPosition, row: usize, col: usize) -> bool {
    if board.max_player && board.board[row][col] == PLAYER_BLACK {
        return true;
    }
    if !board.max_player && board.board[row][col] == PLAYER_WHITE {
        return true;
    }

    false
}

fn isOwnSquare(board: &OthelloPosition, row: usize, col: usize) -> bool {
   if !board.max_player && board.board[row][col] == PLAYER_BLACK {
       return true;
   } 
   if board.max_player && board.board[row][col] == PLAYER_WHITE {
       return true;
   }

   false
}

fn isCandidate(board: &OthelloPosition, row: usize, col: usize) -> bool {
    if !isFree(board, row, col) {
        return false;
    }
    if hasNeighbor(board, row, col) {
        return true;
    }

    false
}


fn hasNeighbor(board: &OthelloPosition, row: usize, col: usize) -> bool{
    !isFree(board, row - 1, col) || !isFree(board, row - 1, col + 1) || !isFree(board, row, col + 1) || !isFree(board, row + 1, col + 1) || !isFree(board, row + 1, col) || !isFree(board, row + 1, col - 1) || !isFree(board, row, col - 1) || !isFree(board, row - 1, col - 1)
}

fn isFree(board: &OthelloPosition, row: usize, col: usize) -> bool {
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

// Compares two vectors to see if they differ
fn vec_differs(fst: &[char], snd: &[char]) -> (bool, isize, char) {
  for i in (0..fst.len()) {
    if fst[i] != snd[i] {
      return (true, i as isize, snd[i]);
    }
  }

  (false, -1, EMPTY_CELL)
}