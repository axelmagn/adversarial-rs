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
pub struct TicTacState<'a> {
    /// The tic tac toe board
    pub board: [TicTacCell; 9],
    /// The player whose turn it is
    pub player: TicTacPlayer,
    /// The predecessor state
    pub parent: Option<&'a TicTacState<'a>>
}


impl<'a> TicTacState<'a> {
    /// Get the state for a new game
    fn new_game() -> TicTacState<'a> {
        TicTacState {
            board: [TicTacCell::Empty; 9],
            player: TicTacPlayer::X,
            parent: None,
        }
    }

    fn print(&self) {
        for row in 0..3 {
            for col in 0..3 {
                let s = match self.board[row*3+col] {
                    TicTacCell::X => "X",
                    TicTacCell::O => "O",
                    TicTacCell::Empty => " ",
                };
                print!("{}", s);
            }
            print!("\n");
        }
    }

    fn get_winner(&self) -> Option<TicTacPlayer> {
        // test columns
        for col in 0..3 {
            let first_cell = self.board[col];
            if first_cell == TicTacCell::Empty { continue; }
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


impl<'a> State<'a, TicTacPlayer, TicTacAction> for TicTacState<'a> {
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
    fn result(&'a self, action: TicTacAction) -> Result<Self, ActionError> {
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
        out.player = match action.player {
            TicTacPlayer::X => TicTacPlayer::O,
            TicTacPlayer::O => TicTacPlayer::X,
        };
        out.parent = Some(&self);
        Ok(out)
    }

    /// Get the utility of the state
    fn utility(&self, player: TicTacPlayer) -> f64 {
        match self.get_winner() {
            None => 0.0,
            Some(winner) => {
                if player == winner {
                    1.0
                } else {
                    -1.0
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
    use ai::*;

    #[test]
    fn test_state_new_game() {
        let t = TicTacState::new_game();
        assert_eq!(TicTacPlayer::X, t.player);
        for i in 0..9 {
            assert_eq!(TicTacCell::Empty, t.board[i]);
        }
    }

    #[test]
    fn test_state_terminations() {
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
            assert_eq!(1.0,   t.utility(TicTacPlayer::O));
            assert_eq!(-1.0,  t.utility(TicTacPlayer::X));
            assert!(t.terminal());
        }

        // test column win for X
        for j in 0..3 {
            let mut t = TicTacState::new_game();
            for i in 0..3 {
                t.board[i*3+j] = TicTacCell::X;
            }
            let winner = t.get_winner().unwrap();
            assert_eq!(TicTacPlayer::X, winner);
            assert_eq!(1.0,   t.utility(TicTacPlayer::X));
            assert_eq!(-1.0,  t.utility(TicTacPlayer::O));
            assert!(t.terminal());
        }

        // test row win for O
        for j in 0..3 {
            let mut t = TicTacState::new_game();
            for i in 0..3 {
                t.board[j*3+i] = TicTacCell::O;
            }
            let winner = t.get_winner().unwrap();
            assert_eq!(TicTacPlayer::O, winner);
            assert_eq!(1.0,   t.utility(TicTacPlayer::O));
            assert_eq!(-1.0,  t.utility(TicTacPlayer::X));
            assert!(t.terminal());
        }
        
        // test row win for X
        for j in 0..3 {
            let mut t = TicTacState::new_game();
            for i in 0..3 {
                t.board[j*3+i] = TicTacCell::X;
            }
            let winner = t.get_winner().unwrap();
            assert_eq!(TicTacPlayer::X, winner);
            assert_eq!(1.0,   t.utility(TicTacPlayer::X));
            assert_eq!(-1.0,  t.utility(TicTacPlayer::O));
            assert!(t.terminal());
        }
    }

    #[test]
    fn test_state_actions() {
        // test that new board contains all actions
        {
            let t = TicTacState::new_game();
            let actions = t.actions();
            for i in 0..9 {
                let ax = TicTacAction { position: i, player: TicTacPlayer::X, };
                let ao = TicTacAction { position: i, player: TicTacPlayer::O, };
                assert!(actions.contains(&ax));
                assert!(!actions.contains(&ao));
            }
        }

        // test that a board with a nonempty square does not contain an action
        // there
        for i in 0..9 {
            let mut t = TicTacState::new_game();
            t.board[i] = TicTacCell::X;
            t.player = TicTacPlayer::O;
            let actions = t.actions();
            let ax = TicTacAction { position: i, player: TicTacPlayer::X, };
            let ao = TicTacAction { position: i, player: TicTacPlayer::O, };
            assert!(!actions.contains(&ax));
            assert!(!actions.contains(&ao));
        }
    }

    #[test]
    fn test_state_result() {
        let t0 = TicTacState::new_game();

        let a1 = TicTacAction { position: 4, player: TicTacPlayer::X };
        let t1 = t0.result(a1).unwrap();
        assert_eq!(t1.board[4], TicTacCell::X);
        assert_eq!(t1.player, TicTacPlayer::O);
        
        let a2 = TicTacAction { position: 0, player: TicTacPlayer::O };
        let t2 = t1.result(a2).unwrap();
        assert_eq!(t2.board[0], TicTacCell::O);
        assert_eq!(t2.player, TicTacPlayer::X);

        let a3 = TicTacAction { position: 3, player: TicTacPlayer::X };
        let t3 = t2.result(a3).unwrap();
        assert_eq!(t3.board[3], TicTacCell::X);
        assert_eq!(t3.player, TicTacPlayer::O);

        // test parents
        let t3p = match t3.parent {
            Some(p) => p,
            None => panic!("Child state has no parent"),
        };
        assert_eq!(&t2, t3p);
        let t2p = match t2.parent {
            Some(p) => p,
            None => panic!("Child state has no parent"),
        };
        assert_eq!(&t1, t2p);
    }
}
