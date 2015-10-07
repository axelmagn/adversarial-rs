extern crate adversarial;
use adversarial::tictac::*;
use adversarial::ai::*;

fn main() {
    let t = TicTacState::new_game();
    println!("Simulating minimax game");
    let a = TicTacAction { position: 0, player: TicTacPlayer::X };
    let mut t = t.result(a).unwrap();
    while !t.terminal() {
        t.print();
        print!("\n");
        let best_action = t.minimax_search().unwrap();
        t = t.result(best_action).unwrap();
    }
    println!("Solution Found!");
    t.print();
    print!("\n");
}
