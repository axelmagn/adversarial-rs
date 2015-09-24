/// General AI implementations for adversarial games

pub enum ActionError {
    IllegalAction,
}

/// A Game State for use with the AI
pub trait State<P, A> {
    /// Get the player who can move in the state
    fn player(&self) -> P;

    /// Get the possible actions for the current player in the state
    fn actions(&self) -> Vec<A>;

    /// Get the result an action in the state
    fn result(&self, action: A) -> Result<Self, ActionError>;

    /// Get the utility of the state
    fn utility(&self, player: P) -> i32;

    /// Return true if the state is terminal
    fn terminal(&self) -> bool;
}

/// A game state
pub trait Game<S> where S: State {
    fn initial_state(&self) -> S;
}
