extern crate adversarial;

use adversarial::tictac::*;
use adversarial::ai::*;
use std::io;

fn main() {
    println!("Playing Tic Tac Toe");
    // get player
    println!("Do you want to play as X or O? ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let player = match buffer[..].trim() {
        "X" => TicTacPlayer::X,
        "O" => TicTacPlayer::O,
        _ => panic!("Unknown player"),
    };
    
    // initialize game
    let mut t = TicTacState::new_game();
    while !t.terminal() {
        t.print();
        println!("\n");
        let actions = t.actions();
        let action = if t.player == player {
            // if it's the player's turn, parse their move
            let mut valid_action = false;
            let mut player_action = None;
            while !valid_action {
                println!("What position would like to play? [0-8] ");
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer).unwrap();
                let p: usize = (&buffer).trim().parse().unwrap();
                let a = TicTacAction {
                    position: p,
                    player: player
                };
                if actions.contains(&a) {
                    player_action = Some(a);
                    valid_action = true;
                } else {
                    println!("Invalid Move!");
                }
            }
            player_action.unwrap()
        } else {
            // otherwise, let the ai move
            t.minimax_search().unwrap()
        };
        t = t.result(action).unwrap();
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
    println!("\n");
}
