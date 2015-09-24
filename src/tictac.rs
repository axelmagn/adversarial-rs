/// Tic Tac Toe Game definition


use ai::{State, Game, ActionError};


/// A single board cell
#[derive(Copy,Clone,PartialEq,Eq,Debug)]
pub enum TicTacCell {
    X,
    O,
    Empty,
}

impl TicTacCell {
    fn player(&self) -> Option<TicTacPlayer> {
        match *self {
            TicTacCell::X => Some(TicTacPlayer::X),
            TicTacCell::O => Some(TicTacPlayer::O),
            TicTacCell::Empty => None,
        }
    }
}


/// A tic tac toe player
#[derive(Copy,Clone,PartialEq,Eq,Debug)]
pub enum TicTacPlayer {
    X,
    O,
}

impl TicTacPlayer {
    fn cell(&self) -> TicTacCell {
        match *self {
            TicTacPlayer::X => TicTacCell::X,
            TicTacPlayer::O => TicTacCell::O,
        }
    }
}


/// A tic tac toe game state
#[derive(Copy,Clone,PartialEq,Eq,Debug)]
pub struct TicTacAction {
    position: usize,
    player: TicTacPlayer,
}


/// A tic tac toe game state
#[derive(Copy,Clone,PartialEq,Eq,Debug)]
pub struct TicTacState {
    /// The tic tac toe board
    pub board: [TicTacCell; 9],
    /// The player whose turn it is
    pub player: TicTacPlayer,
}


impl TicTacState {
    /// Get the state for a new game
    fn new_game() -> TicTacState {
        TicTacState {
            board: [TicTacCell::Empty; 9],
            player: TicTacPlayer::X,
        }
    }

    fn get_winner(&self) -> Option<TicTacPlayer> {
        // test columns
        for col in 0..3 {
            let first_cell = self.board[col];
            if first_cell == TicTacCell::Empty { break; }
            let mut win = true;
            for row in 1..3 {
                if self.board[row*3+col] != first_cell {
                    win = false;
                    break;
                }
            }
            if win {
                return first_cell.player();
            }
        }

        // test rows
        for row in 0..3 {
            let first_cell = self.board[row*3];
            if first_cell == TicTacCell::Empty { continue; }
            let mut win = true;
            for col in 1..3 {
                if self.board[row*3+col] != first_cell {
                    win = false;
                    break;
                }
            }
            if win {
                return first_cell.player();
            }

        }

        // test diagonals
        let first_cell = self.board[0];
        if first_cell != TicTacCell::Empty { 
            let mut win = true;
            for i in 1..3 {
                if self.board[i*3+i] != first_cell {
                    win = false;
                    break;
                }
            }
            if win {
                return first_cell.player();
            }
        }

        let first_cell = self.board[2];
        if first_cell != TicTacCell::Empty { 
            let mut win = true;
            for i in 1..3 {
                if self.board[i*3+3-i] != first_cell {
                    win = false;
                    break;
                }
            }
            if win {
                return first_cell.player();
            }
        }

        None
    }

}


impl State<TicTacPlayer, TicTacAction> for TicTacState {
    /// Get the player who can move in the state
    fn player(&self) -> TicTacPlayer { self.player }

    /// Get the possible actions for the current player in the state
    /// 
    /// The current player can move onto any empty square
    fn actions(&self) -> Vec<TicTacAction> {
        let mut out = Vec::new();
        for i in 0..9 {
            let cell = self.board[i];
            if cell == TicTacCell::Empty {
                let action = TicTacAction {
                    position: i,
                    player: self.player,
                };
                out.push(action);
            }
        }
        out
    }

    /// Get the result an action in the state
    fn result(&self, action: TicTacAction) -> Result<Self, ActionError> {
        if action.player != self.player {
            return Err(ActionError::WrongPlayer);
        }
        if action.position >= 9 {
            return Err(ActionError::InvalidMove);
        }
        if self.board[action.position] != TicTacCell::Empty {
            return Err(ActionError::IllegalMove);
        }
        let mut out = self.clone();
        out.board[action.position] = action.player.cell();
        Ok(out)
    }

    /// Get the utility of the state
    fn utility(&self, player: TicTacPlayer) -> i32 {
        match self.get_winner() {
            None => 0,
            Some(winner) => {
                if player == winner {
                    1
                } else {
                    -1
                }
            },
        }
    }

    /// Return true if the state is terminal
    fn terminal(&self) -> bool {
        match self.get_winner() {
            None => false,
            Some(_) => true,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_new_game() {
        let t = TicTacState::new_game();
        assert_eq!(TicTacPlayer::X, t.player);
        for i in 0..9 {
            assert_eq!(TicTacCell::Empty, t.board[i]);
        }
    }

    #[test]
    fn test_state_winner() {
        let t = TicTacState::new_game();
        assert_eq!(None, t.get_winner());

        // test column win for O
        for j in 0..3 {
            let mut t = TicTacState::new_game();
            for i in 0..3 {
                t.board[i*3+j] = TicTacCell::O;
            }
            let winner = t.get_winner().unwrap();
            assert_eq!(TicTacPlayer::O, winner);
        }

        // test column win for X
        for j in 0..3 {
            let mut t = TicTacState::new_game();
            for i in 0..3 {
                t.board[i*3+j] = TicTacCell::X;
            }
            let winner = t.get_winner().unwrap();
            assert_eq!(TicTacPlayer::O, winner);
        }

        // test row win for O
        for j in 0..3 {
            let mut t = TicTacState::new_game();
            for i in 0..3 {
                t.board[j*3+i] = TicTacCell::O;
            }
            let winner = t.get_winner().unwrap();
            assert_eq!(TicTacPlayer::O, winner);
        }
        
        // test row win for X
        for j in 0..3 {
            let mut t = TicTacState::new_game();
            for i in 0..3 {
                t.board[j*3+i] = TicTacCell::X;
            }
            let winner = t.get_winner().unwrap();
            assert_eq!(TicTacPlayer::X, winner);
        }
    }
}
