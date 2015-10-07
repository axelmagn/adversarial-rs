extern crate adversarial;
extern crate rand;

use adversarial::tictac::*;
use adversarial::ai::*;
use rand::Rng;

fn main() {
    let t = TicTacState::new_game();
    println!("Simulating minimax game");
    let mut rng = rand::thread_rng();
    let pos: usize = rng.gen::<usize>() % 9;
    let a = TicTacAction { position: pos, player: TicTacPlayer::X };
    let mut t = t.result(a).unwrap();
    while !t.terminal() {
        t.print();
        print!("\n");
        let best_action = t.minimax_search().unwrap();
        t = t.result(best_action).unwrap();
    }
    match t.get_winner() {
        None => {
            println!("DRAW");
        },
        Some(p) => {
            match p {
                TicTacPlayer::X => println!("X Wins!"),
                TicTacPlayer::O => println!("O Wins!"),

            };
        },
    };
    t.print();
    print!("\n");
}
