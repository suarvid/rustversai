use crate::move_generator::{get_moves, Move};
pub const PLAYER_WHITE: char = 'O';
pub const PLAYER_BLACK: char = 'X';
pub const EMPTY_CELL: char = 'E';
pub const WHITE_STRING_REP: char = 'W';
pub const BLACK_STRING_REP: char = 'B';
pub const BOARD_SIZE: usize = 8;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct OthelloPosition {
    pub board: [[char; 10]; 10],
    pub max_player: bool,
    pub score: isize,
}

impl OthelloPosition {
    pub fn empty() -> OthelloPosition {
        let board = [[EMPTY_CELL; 10]; 10];

        OthelloPosition {
            board,
            max_player: true,
            score: 0,
        }
    }

    pub fn worst_for_max() -> OthelloPosition {
        let board = [[PLAYER_BLACK; 10]; 10];
        OthelloPosition {
            board,
            max_player: true,
            score: 0,
        }
    }

    pub fn worst_for_min() -> OthelloPosition {
        let board = [[PLAYER_WHITE; 10]; 10];
        OthelloPosition {
            board,
            max_player: false,
            score: 0,
        }
    }

    pub fn is_game_over(&self) -> bool {
        for row in 1..=8 {
            for col in 1..=8 {
                if self.board[row][col] == 'E' {
                    return false;
                }
            }
        }

        true
    }

    pub fn new(string_rep: &str) -> OthelloPosition {
        if string_rep.len() != 65 {
            OthelloPosition::empty()
        } else {
            let mut board = [[EMPTY_CELL; 10]; 10];
            let mut max_player = false;
            if string_rep.chars().collect::<Vec<char>>()[0] == WHITE_STRING_REP {
                max_player = true;
            }
            for i in 1..=64 {
                let c = string_rep.chars().collect::<Vec<char>>()[i];
                let col = ((i - 1) % BOARD_SIZE) + 1;
                let row = (i - 1) / 8 + 1;
                board[row][col] = c;
            }
            OthelloPosition {
                board,
                max_player,
                score: 0,
            }
        }
    }

    pub fn string_rep(&self) -> String {
        let mut player_char = BLACK_STRING_REP;
        if self.max_player {
            player_char = WHITE_STRING_REP;
        }
        let mut to_return = String::from(player_char);
        for row in 1..=BOARD_SIZE {
            for col in 1..=BOARD_SIZE {
                to_return.push_str(&format!("{}", self.board[row][col]));
            }
        }

        to_return
    }

    pub fn add_piece(&self, row: usize, col: usize, player: char) -> OthelloPosition {
        let mut new_position = OthelloPosition {
            board: self.board.clone(),
            max_player: !self.max_player,
            score: 0,
        };
        new_position.board[row][col] = player;
        new_position
    }


    pub fn generate_children(&self) -> Vec<OthelloPosition> {
        let possible_moves = get_moves(&self);
        let mut child_boards = Vec::new();

        for p_move in possible_moves {
            child_boards.push(Move::make_move(&self, &p_move));
        }

        child_boards
    }
}
