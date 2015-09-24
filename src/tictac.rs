/// Tic Tac Toe Game definition

use ai;

pub enum TicTacCell {
    X,
    O,
    Empty,
}

pub enum TicTacPlayer {
    X,
    O,
}

pub struct TicTacState {
    /// The tic tac toe board
    pub board: [TicTacCell;9],
    /// The player whose turn it is
    pub player: u8,
}

impl TicTacState {
    /// Get the state for a new game
    fn new_game() -> TicTacState {
        TicTacState {
            board: [TicTacCell::Empty; 9],
            player: TicTacPlayer::X,
        }
    }
}
