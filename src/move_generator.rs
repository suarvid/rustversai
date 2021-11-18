pub const PLAYER_WHITE: char = 'O';
pub const PLAYER_BLACK: char = 'X';
pub const EMPTY_CELL: char = 'E';
pub const BOARD_SIZE: usize = 8;
//TODO: Somewhere there is a mixup between PLAYER_WHITE and PLAYER_BLACK

#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    player: char,
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct OthelloPosition {
    pub board: [[char; 10]; 10],
    pub max_player: bool,
}

impl OthelloPosition {
    pub fn empty() -> OthelloPosition {
        let board = [[EMPTY_CELL; 10]; 10];

        OthelloPosition {
            board,
            max_player: true,
        }
    }

    pub fn worst_for_max() -> OthelloPosition {
        let board = [[PLAYER_BLACK; 10]; 10];
        OthelloPosition {
            board,
            max_player: true,
        }
    }

    pub fn worst_for_min() -> OthelloPosition {
        let board = [[PLAYER_WHITE; 10]; 10];
        OthelloPosition {
            board,
            max_player: false,
        }
    }

    pub fn is_empty(&self) -> bool {
        for row in &self.board {
            for c in row {
                if *c != EMPTY_CELL {
                    return true;
                }
            }
        }

        false
    }

    pub fn new(string_rep: &str) -> OthelloPosition {
        if string_rep.len() != 65 {
            OthelloPosition::empty()
        } else {
            let mut board = [[EMPTY_CELL; 10]; 10];
            let mut max_player = false;
            if string_rep.chars().collect::<Vec<char>>()[0] == 'W' {
                max_player = true;
            }
            for i in 1..=64 {
                let c = string_rep.chars().collect::<Vec<char>>()[i];
                let col = ((i - 1) % BOARD_SIZE) + 1;
                let row = (i - 1) / 8 + 1;
                board[row][col] = c;
            }
            OthelloPosition { board, max_player }
        }
    }

    pub fn add_piece(&self, row: usize, col: usize, player: char) -> OthelloPosition {
        let mut new_position = OthelloPosition {
            board: self.board.clone(),
            max_player: !self.max_player,
        };
        new_position.board[row][col] = player;
        new_position
    }

    pub fn print(&self) {
        for row in 0..self.board.len() {
            for col in 0..self.board.len() {
                print!("{}", self.board[row][col]);
            }
            println!();
        }
    }
}

impl Move {
    pub fn new(player: char, row: usize, col: usize) -> Move {
        Move { player, row, col }
    }

    pub fn make_move<'a>(
        board: &OthelloPosition,
        to_make: &Option<Move>,
    ) -> Option<OthelloPosition> {
        match to_make {
            Some(m) => Some(board.add_piece(m.row + 1, m.col + 1, m.player)), //TODO: Might have to remove the +1's
            None => None,
        }
    }
}

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
        if is_free(board, row - i as usize, col - i as usize ) {
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

// This does not seem to work!
// This might actually work! Problem is, invalid move is made!
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
    for i in 0..fst.len() {
        if fst[i] != snd[i] {
            return (true, i as isize, snd[i]);
        }
    }

    (false, -1, EMPTY_CELL)
}
