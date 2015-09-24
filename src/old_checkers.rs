use std::ops::{Index,IndexMut};

const SQUARE_BIT_SIZE: u8 = 2;
const SQUARE_BIT_MASK: u64 = (1 << (SQUARE_BIT_SIZE-1)) - 1;
const BOARD_HEIGHT: usize = 8;
const BOARD_WIDTH: usize = 4; // every other square

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum BoardSquareState {
    Empty,
    White,
    Black,
    WhiteKing,
    BlackKing,
}

// A board square index on an 8x8 board
struct BoardSquareIdx(usize, usize);
struct BoardStorageIdx(usize, usize);

// Assumes 8x8 board, with checkers aligned so that the bottom left corner is a
// legal position
#[derive(Copy, Clone)]
struct BoardState {
    // encode board as 8 high, 4 wide, 4 state bit array organize by row-first
    // index, where bottom left corner is (0,0)
    state: [BoardSquareState; BOARD_HEIGHT * BOARD_WIDTH],
}

impl BoardState {
    pub fn new_empty() -> Self {
        BoardState { state: [BoardSquareState::Empty; BOARD_HEIGHT * BOARD_WIDTH] }
    }

    pub fn new() -> Self {
        let mut board = BoardState::new_empty();
        for row in 0..3 {
            for col in 0..4 {
                board[(row, col)] = White;
            }
        }
        for row in 5..8 {
            for col in 0..4 {
                board[(row, col)] = Black;
            }
        }
        board
    }

    pub fn is_valid_move(&self, src: BoardSquareIdx(usize, usize), 
                         dst: BoardSquareIdx(usize, usize)) -> bool {
    }


}

impl Index<(usize, usize)> for BoardState {
    type Output = BoardSquareState;

    fn index<'a>(&'a self, _index: (usize, usize)) -> &'a BoardSquareState {
        let (row, col) = _index;
        &self.state[row * BOARD_WIDTH + col]
    }
}

impl IndexMut<(usize, usize)> for BoardState {
    fn index_mut<'a>(&'a mut self, _index: (usize, usize)) -> &'a mut 
            BoardSquareState {
        let (row, col) = _index;
        &mut self.state[row * BOARD_WIDTH + col]
    }
}

impl Index<BoardSquareIdx(usize, usize)> for BoardState {
    type Output = BoardSquareState;

    fn index<'a>(&'a self, _index: (usize, usize)) -> &'a BoardSquareState {
        let (row, col) = _index;
        &self.state[row * BOARD_WIDTH + col]
    }
}

impl IndexMut<BoardSquareIdx(usize, usize)> for BoardState {
    fn index_mut<'a>(&'a mut self, _index: (usize, usize)) -> &'a mut 
            BoardSquareState {
        let (row, col) = _index;
        &mut self.state[row * BOARD_WIDTH + col]
    }
}


#[cfg(tests)]
mod tests {

    #[test]
    fn test_boardstate() {
        let mut b = BoardState::new_empty();
        for i in 0..BOARD_HEIGHT {
            for j in 0..BOARD_WIDTH {
                assert_eq!(BoardSquareState::Empty, b[(i, j)]);
                b[(i, j)] = BoardSquareState::BlackKing;
                assert_eq!(BoardSquareState::BlackKing, b[(i, j)]);
            }
        }
    }
}
