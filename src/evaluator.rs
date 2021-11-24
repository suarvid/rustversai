/// This file contains the implementation of the struct Evaluator
/// and its associated functions. These are used for evaluating boards,
/// where negative values represent situations beneficial for the min player
/// and positive values represent situations beneficial for the max player.
/// A value of zero means the advantages of the players is in balance.

use crate::board::{OthelloPosition, EMPTY_CELL, PLAYER_BLACK, PLAYER_WHITE};
use crate::move_generator;

/// Represents an Evaluator with its associated weights.
/// Each weight determines how much each aspect taken into
/// consideration should affect the evaluated value of a given board.
pub struct Evaluator {
    pub count_weight: isize,
    pub corners_weight: isize,
    pub imm_mobility_weight: isize,
    pub pot_mobility_weight: isize,
    pub corner_adjacent_weight: isize,
}

impl Evaluator {
    /// Returns a new evaluator with weights deemed to
    /// lead to good performance, at least by experimentation.
    pub fn default() -> Evaluator {
        Evaluator {
            count_weight: -100,
            corners_weight: 4000,
            imm_mobility_weight: 400,
            pot_mobility_weight: 600,
            corner_adjacent_weight: 400,
        }
    }

    /// Evaluates a given board. Returns an integer representing
    /// which player is deemed to have the advantage and the
    /// magnitude of that advantage.
    /// 
    /// # Arguments
    /// 
    /// * `board` - An OthelloPosition representing the board to be evaluated.
    pub fn evaluate(&self, board: &OthelloPosition) -> isize {
        self.count_weight * Evaluator::piece_count_value(board)
            + self.corners_weight * Evaluator::corners_value(board)
            + self.imm_mobility_weight * Evaluator::immediate_mobility(board)
            + self.pot_mobility_weight * Evaluator::potential_mobility(board)
            + self.corner_adjacent_weight * Evaluator::giving_away_corners(board)
    }

    /// Counts the number of pieces belonging to each player.
    /// Returns a normalized value representing the difference in
    /// piece count.
    /// 
    /// # Arguments
    /// 
    /// * `board` - An OthelloPosition representing the board.
    fn piece_count_value(board: &OthelloPosition) -> isize {
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

    /// Counts the occurrences of pieces belonging to each player
    /// in the squares immediately adjacent to a corner.
    /// Returns a negative value if the max player has more pieces in
    /// these squares, a positive value if the min player has more.
    /// Returns a value of 0 if no such squares are taken or if each player
    /// holds the same amount of these squares.
    /// 
    /// # Arguments
    /// 
    /// * `board` - An OthelloPosition representing the board
    fn giving_away_corners(board: &OthelloPosition) -> isize {
        let mut white_value = 0;
        let mut black_value = 0;
        match board.board[2][2] {
            PLAYER_WHITE => white_value += 1,
            PLAYER_BLACK => black_value += 1,
            _ => (),
        };
        match board.board[2][7] {
            PLAYER_WHITE => white_value += 1,
            PLAYER_BLACK => black_value += 1,
            _ => (),
        };
        match board.board[7][2] {
            PLAYER_WHITE => white_value += 1,
            PLAYER_BLACK => black_value += 1,
            _ => (),
        }
        match board.board[7][7] {
            PLAYER_WHITE => white_value += 1,
            PLAYER_BLACK => black_value += 1,
            _ => (),
        }
        match board.board[2][1] {
            PLAYER_WHITE => white_value += 1,
            PLAYER_BLACK => black_value += 1,
            _ => (),
        }
        match board.board[1][2] {
            PLAYER_WHITE => white_value += 1,
            PLAYER_BLACK => black_value += 1,
            _ => (),
        }
        match board.board[1][7] {
            PLAYER_WHITE => white_value += 1,
            PLAYER_BLACK => black_value += 1,
            _ => (),
        }
        match board.board[2][8] {
            PLAYER_WHITE => white_value += 1,
            PLAYER_BLACK => black_value += 1,
            _ => (),
        }
        match board.board[7][1] {
            PLAYER_WHITE => white_value += 1,
            PLAYER_BLACK => black_value += 1,
            _ => (),
        }
        match board.board[7][8] {
            PLAYER_WHITE => white_value += 1,
            PLAYER_BLACK => black_value += 1,
            _ => (),
        }
        match board.board[8][2] {
            PLAYER_WHITE => white_value += 1,
            PLAYER_BLACK => black_value += 1,
            _ => (),
        }
        match board.board[8][7] {
            PLAYER_WHITE => white_value += 1,
            PLAYER_BLACK => black_value += 1,
            _ => (),
        }
        if white_value + black_value != 0 {
            return -(100 * (white_value - black_value) / (white_value + black_value));
        }

        0
    }

    /// Counts the number of empty squares next to each piece belonging
    /// to either player.
    /// 
    /// # Arguments
    /// 
    /// * `board` - An OthelloPosition representing the board.
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
                    if row == 1 {
                        // Can't look up
                        if col == 1 {
                            // Can't look left
                            if right == EMPTY_CELL {
                                white_count += 1
                            };
                            if below == EMPTY_CELL {
                                white_count += 1
                            };
                            if bot_right == EMPTY_CELL {
                                white_count += 1
                            };
                        } else if col == 8 {
                            // Can't look right
                            if left == EMPTY_CELL {
                                white_count += 1
                            };
                            if below == EMPTY_CELL {
                                white_count += 1
                            };
                            if bot_left == EMPTY_CELL {
                                white_count += 1
                            };
                        } else {
                            if left == EMPTY_CELL {
                                white_count += 1
                            };
                            if right == EMPTY_CELL {
                                white_count += 1
                            };
                            if bot_left == EMPTY_CELL {
                                white_count += 1
                            };
                            if below == EMPTY_CELL {
                                white_count += 1
                            };
                            if bot_right == EMPTY_CELL {
                                white_count += 1
                            };
                        }
                    } else if row == 8 {
                        // Can't look down
                        if col == 1 {
                            // Can't look left
                            if above == EMPTY_CELL {
                                white_count += 1
                            };
                            if top_right == EMPTY_CELL {
                                white_count += 1
                            };
                            if right == EMPTY_CELL {
                                white_count += 1
                            };
                        } else if col == 8 {
                            // Can't look right
                            if above == EMPTY_CELL {
                                white_count += 1
                            };
                            if top_left == EMPTY_CELL {
                                white_count += 1
                            };
                            if left == EMPTY_CELL {
                                white_count += 1
                            };
                        } else {
                            if top_left == EMPTY_CELL {
                                white_count += 1
                            };
                            if top_right == EMPTY_CELL {
                                white_count += 1
                            };
                            if left == EMPTY_CELL {
                                white_count += 1
                            };
                            if right == EMPTY_CELL {
                                white_count += 1
                            };
                        }
                    } else {
                        // Can look both up and down
                        if above == EMPTY_CELL {
                            white_count += 1
                        };
                        if below == EMPTY_CELL {
                            white_count += 1
                        };
                        if col == 1 {
                            // Cannot look left
                            if top_right == EMPTY_CELL {
                                white_count += 1
                            };
                            if right == EMPTY_CELL {
                                white_count += 1
                            };
                            if bot_right == EMPTY_CELL {
                                white_count += 1
                            };
                        } else if col == 8 {
                            // Cannot look right
                            if top_left == EMPTY_CELL {
                                white_count += 1
                            };
                            if left == EMPTY_CELL {
                                white_count += 1
                            };
                            if bot_left == EMPTY_CELL {
                                white_count += 1
                            };
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
                    if row == 1 {
                        // Can't look up
                        if col == 1 {
                            // Can't look left
                            if right == EMPTY_CELL {
                                black_count += 1
                            };
                            if below == EMPTY_CELL {
                                black_count += 1
                            };
                            if bot_right == EMPTY_CELL {
                                black_count += 1
                            };
                        } else if col == 8 {
                            // Can't look right
                            if left == EMPTY_CELL {
                                black_count += 1
                            };
                            if below == EMPTY_CELL {
                                black_count += 1
                            };
                            if bot_left == EMPTY_CELL {
                                black_count += 1
                            };
                        } else {
                            if left == EMPTY_CELL {
                                black_count += 1
                            };
                            if right == EMPTY_CELL {
                                black_count += 1
                            };
                            if bot_left == EMPTY_CELL {
                                black_count += 1
                            };
                            if below == EMPTY_CELL {
                                black_count += 1
                            };
                            if bot_right == EMPTY_CELL {
                                black_count += 1
                            };
                        }
                    } else if row == 8 {
                        // Can't look down
                        if col == 1 {
                            // Can't look left
                            if above == EMPTY_CELL {
                                black_count += 1
                            };
                            if top_right == EMPTY_CELL {
                                black_count += 1
                            };
                            if right == EMPTY_CELL {
                                black_count += 1
                            };
                        } else if col == 8 {
                            // Can't look right
                            if above == EMPTY_CELL {
                                black_count += 1
                            };
                            if top_left == EMPTY_CELL {
                                black_count += 1
                            };
                            if left == EMPTY_CELL {
                                black_count += 1
                            };
                        } else {
                            if top_left == EMPTY_CELL {
                                black_count += 1
                            };
                            if top_right == EMPTY_CELL {
                                black_count += 1
                            };
                            if left == EMPTY_CELL {
                                black_count += 1
                            };
                            if right == EMPTY_CELL {
                                black_count += 1
                            };
                        }
                    } else {
                        // Can look both up and down
                        if above == EMPTY_CELL {
                            black_count += 1
                        };
                        if below == EMPTY_CELL {
                            black_count += 1
                        };
                        if col == 1 {
                            // Cannot look left
                            if top_right == EMPTY_CELL {
                                black_count += 1
                            };
                            if right == EMPTY_CELL {
                                black_count += 1
                            };
                            if bot_right == EMPTY_CELL {
                                black_count += 1
                            };
                        } else if col == 8 {
                            // Cannot look right
                            if top_left == EMPTY_CELL {
                                black_count += 1
                            };
                            if left == EMPTY_CELL {
                                black_count += 1
                            };
                            if bot_left == EMPTY_CELL {
                                black_count += 1
                            };
                        }
                    }
                }
            }
        }
        if white_count + black_count != 0 {
            return 100 * (white_count - black_count) / (white_count + black_count);
        }
        0
    }

    /// Counts the number of moves available to each player if it were their
    /// turn with the given pieces on the board.
    /// 
    /// # Arguments
    /// 
    /// * `board` - An OthelloPosition representing the board.
    fn immediate_mobility(board: &OthelloPosition) -> isize {
        let num_max_moves;
        let num_min_moves;
        if board.max_player {
            let min_board = OthelloPosition {
                board: board.board,
                max_player: false,
                score: 0,
            };
            let max_board = board;
            num_max_moves = move_generator::get_moves(max_board).len() as isize;
            num_min_moves = move_generator::get_moves(&min_board).len() as isize;
        } else {
            let max_board = OthelloPosition {
                board: board.board,
                max_player: true,
                score: 0,
            };
            let min_board = board;
            num_max_moves = move_generator::get_moves(&max_board).len() as isize;
            num_min_moves = move_generator::get_moves(min_board).len() as isize;
        }

        if num_max_moves + num_min_moves != 0 {
            return (100 * (num_max_moves - num_min_moves) / (num_max_moves + num_min_moves))
                as isize;
        }

        0
    }


    /// Counts the number of corners held by each player.
    /// Returns a negative value if the min player holds more corners,
    /// a positive value if the max player holds more corners.
    /// If no corners are held, a value of 0 is returned.
    /// 
    /// # Arguments
    /// 
    /// * `board` - An OthelloPosition representing the board.
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
}
