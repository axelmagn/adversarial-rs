/// General AI implementations for adversarial games


use std::f64;


#[derive(Copy,Clone,PartialEq,Eq,Debug)]
pub enum ActionError {
    /// This move does not make sense on any board
    InvalidMove,
    /// This move makes sense but violates the rules of the game
    IllegalMove,
    /// This move is out of turn
    WrongPlayer,
}

/// A Game State for use with the AI
pub trait State<'a, P, A> : Sized {
    /// Get the player who can move in the state
    fn player(&self) -> P;

    /// Get the possible actions for the current player in the state
    fn actions(&self) -> Vec<A>;

    /// Get the result an action in the state
    fn result(&'a self, action: A) -> Result<Self, ActionError>;

    /// Get the utility of the state
    fn utility(&self, player: P) -> f64;

    /// Return true if the state is terminal
    fn terminal(&self) -> bool;

    /// Perform a minimax search
    fn minimax_search(&self) -> A {
        let mut best_action = None;
        let mut best_score = f64::MIN;
        for a in self.actions() {
            let candidate = self.result(a).unwrap(); 
            let score = candidate.minimax_min_value();
            if score > best_score {
                best_action = Some(a);
                best_score = score;
            }
        }
        best_action.unwrap();
    }

    /// Return the highest possible utility that can be achieved from this
    /// state
    fn minimax_max_value(&self) -> f64 {
        if self.terminal() {
            return self.utility(self.player);
        }
        let mut v = f64::MIN;
        for a in self.actions() {
            v = v.max(self.result(a).unwrap().minimax_min_value())
        }
    }

    /// Return the lowest possible utility that can be achieved from this state
    fn minimax_min_value(&self) -> f64 {
        if self.terminal() {
            return self.utility(self.player);
        }
        let mut v = f64::MIN;
        for a in self.actions() {
            v = v.min(self.result(a).unwrap().minimax_max_value())
        }

    }
}

/// A game state
pub trait Game<'a, S, P, A> where S: State<'a, P, A> {
    fn initial_state(&self) -> S;
}
