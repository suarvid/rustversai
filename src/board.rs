/// This file contains the representation of a board used in Othello/Reversi.


use crate::move_generator::{get_moves, Move};
pub const PLAYER_WHITE: char = 'O';
pub const PLAYER_BLACK: char = 'X';
pub const EMPTY_CELL: char = 'E';
pub const WHITE_STRING_REP: char = 'W';
pub const BLACK_STRING_REP: char = 'B';
pub const BOARD_SIZE: usize = 8;


/// The representation of a given Othello position,
/// includes the pieces on the board, the player who is to
/// play next, and the evaluated score of a given board.
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct OthelloPosition {
    /// The pieces on the board. The actual playable area is 8x8, not 10x10.
    pub board: [[char; 10]; 10],
    // Represents which player should go next.
    pub max_player: bool,
    // The evaluated score of this board.
    pub score: isize,
}

impl OthelloPosition {
    /// Returns a new, empty board.
    /// Assumes the max player plays first.
    pub fn empty() -> OthelloPosition {
        let board = [[EMPTY_CELL; 10]; 10];

        OthelloPosition {
            board,
            max_player: true,
            score: 0,
        }
    }

    /// Returns a board representing the worst possible
    /// case for the max player.
    pub fn worst_for_max() -> OthelloPosition {
        let board = [[PLAYER_BLACK; 10]; 10];
        OthelloPosition {
            board,
            max_player: true,
            score: 0,
        }
    }

    /// Returns a board representing the worst possible
    /// case for the min player.
    pub fn worst_for_min() -> OthelloPosition {
        let board = [[PLAYER_WHITE; 10]; 10];
        OthelloPosition {
            board,
            max_player: false,
            score: 0,
        }
    }

    /// Returns a boolean representing whether or not it is possible
    /// for either player to make more moves with the given state of
    /// the current board.
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

    /// Returns an instance of OthelloPosition representing the board
    /// represented in the given string.
    /// 
    /// # Arguments
    /// 
    /// * `string_rep` - A string representation of an Othello board, along with the player to go next.
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

    /// Returns the string representation of an OthelloPosition.
    /// Matches the format used to create an OthelloPosition in new(string_rep: &str).
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

    /// Adds a given piece in the given row and column of the board.
    /// Does not check if this conflicts with the current state of the board.
    /// Does not mutate in-place, returns a brand new OthelloPosition.
    /// 
    /// # Arguments
    /// 
    /// * `row` - Which row to add the piece to.
    /// * `col` - Which column to add the piece to.
    /// * `player` - A char representation of the colour of the piece to be added.
    pub fn add_piece(&self, row: usize, col: usize, player: char) -> OthelloPosition {
        let mut new_position = OthelloPosition {
            board: self.board.clone(),
            max_player: !self.max_player,
            score: 0,
        };
        new_position.board[row][col] = player;
        new_position
    }


    /// Generates the boards reachable in one move from the board.
    pub fn generate_children(&self) -> Vec<OthelloPosition> {
        let possible_moves = get_moves(&self);
        let mut child_boards = Vec::new();

        for p_move in possible_moves {
            child_boards.push(Move::make_move(&self, &p_move));
        }

        child_boards
    }
}
