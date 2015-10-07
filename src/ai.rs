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
pub trait State<P, A> : Sized where A: Copy {
    /// Get the player who can move in the state
    fn player(&self) -> P;

    /// Get the possible actions for the current player in the state
    fn actions(&self) -> Vec<A>;

    /// Get the result an action in the state
    fn result(&self, action: A) -> Result<Self, ActionError>;

    /// Get the utility of the state
    fn utility(&self, player: P) -> f64;

    /// Return true if the state is terminal
    fn terminal(&self) -> bool;

    /// Perform a minimax search
    fn minimax_search(&self) -> Option<A> {
        let mut best_action = None;
        let mut best_score = f64::MIN;
        for a in self.actions() {
            let candidate = self.result(a).unwrap(); 
            let score = -candidate.minimax_value();
            if score > best_score {
                best_action = Some(a);
                best_score = score;
            }
        }
        best_action
    }

    /// Return the maximum value available from this state for the current
    /// player, assuming alternating zero sum game and other player plays 
    /// optimally
    fn minimax_value(&self) -> f64 {
        if self.terminal() {
            return self.utility(self.player());
        }
        let mut v = f64::MIN;
        for a in self.actions() {
            let s = self.result(a).unwrap();
            v = v.max(-s.minimax_value());
        }
        v
    }
}

/// A game state
pub trait Game<S, P, A> where S: State<P, A> {
    fn initial_state(&self) -> S;
}
