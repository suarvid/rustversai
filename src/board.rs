#[derive(Debug, Clone)]
pub struct Board {
    pub cells: Vec<Vec<char>>,
}

impl Board {
    pub fn add_piece(&self, row: usize, col: usize, piece: char) -> Board {
        let mut new_board = Board {
            cells: self.cells.clone(),
        };
        let board_size = new_board.cells.len();

        if row < board_size && col < board_size {
            new_board.cells[row][col] = piece;
        }

        new_board
    }

    fn empty_board(n_rows: usize, n_cols: usize) -> Board {
        let empty_row = vec!['e'; n_cols];
        let cells = vec![empty_row; n_rows];

        Board { cells }
    }

    pub fn start_board() -> Board {
        let mut board = Board::empty_board(8, 8);
        return board
            .add_piece(3, 3, 'w')
            .add_piece(3, 4, 'b')
            .add_piece(4, 3, 'b')
            .add_piece(4, 4, 'w');
    }

    pub fn print(&self) {
        for row in self.cells.iter() {
            let _ = row.iter().map(|c| print!("{} ", c)).collect::<Vec<_>>();
            println!("")
        }
    }

    // TODO: check canvas for how the string is formatted and represented
    pub fn read(string_rep: &str) -> Board {
        assert!(!string_rep.is_empty());
        let mut board = Board::empty_board(8, 8);
        let mut row_number = 0;
        let mut col_number = 0; 
        for i in 0..string_rep.as_bytes().len() {
            if string_rep.as_bytes()[i] == '\n' as u8 {
                row_number += 1;
                col_number = 0;
            } else {
                if string_rep.as_bytes()[i] != 'e' as u8 {
                    board =
                        board.add_piece(row_number, col_number, string_rep.as_bytes()[i] as char);
                }

                col_number += 1;
            }
        }

        board
    }

    // helper function for transposing a matrix
    // allows us to reuse the same functions for rows,columns,diagonals
    pub fn transpose(&self) -> Board {
        let v = &self.cells;
        assert!(!v.is_empty());
        let cells = (0..v[0].len())
            .map(|i| {
                v.iter()
                    .map(|inner| inner[i].clone())
                    .collect::<Vec<char>>()
            })
            .collect();
        Board { cells }
    }
}


// TODO: this should be in board.rs
// if we're not starting from the bottom row, the rightmost column has to be the actual rightmost column
// otherwise, no new diagonals are calculated.
// Returns one diagonal, from top left to bottom right
pub fn get_diagonal_left_to_right(board: &Board, rightmost_column: isize) -> std::vec::Vec<char> {
  assert!(rightmost_column >= 0 && rightmost_column < board.cells.len() as isize);
  let mut elements = Vec::new();
  let mut row_number: isize = board.cells.len() as isize - 1;
  let mut col_number = rightmost_column;

  while row_number >= 0 && col_number < board.cells.len() as isize {
    elements.push(board.cells[row_number as usize][col_number as usize]);
    col_number -= 1;
    row_number -= 1;
  }

  elements
}

// leftmost column should start as board_size
pub fn get_diagonal_right_to_left(board: &Board, leftmost_column: isize) -> std::vec::Vec<char> {
  assert!(leftmost_column >= 0 && leftmost_column < board.cells.len() as isize);
  let mut elements = Vec::new();
  let mut row_number: isize = board.cells.len() as isize - 1;
  let mut col_number = leftmost_column;

  while col_number >= 0 && row_number >= 0 {
    elements.push(board.cells[row_number as usize][col_number as usize]);
    col_number += 1;
    row_number -= 1;
  }

  elements
}