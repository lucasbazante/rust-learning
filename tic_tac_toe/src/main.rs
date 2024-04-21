use std::io::{self, Write};

mod game;
use crate::game::game::*;

fn main() {
    // state of game & lambda utility
    let mut table = Table::new();
    let mut player = Player::X;
    let coord = |x: usize| -> (usize, usize) { ((x - 1) / 3, (x - 1) % 3) };

    println!("Welcome! Input the position on the board you want to play.");
    table.display();
    loop {
        print!("Player {}'s move: ", player.to_char());
        let _ = io::stdout().flush();
        let mut pos = String::new();

        io::stdin()
            .read_line(&mut pos)
            .expect("Failed to read line.");

        let pos: (usize, usize) = match pos.trim().parse() {
            Ok(n) => match n {
                1..=9 => coord(n),
                _ => {
                    println!("Invalid move position! Try again.");
                    continue;
                }
            },
            Err(_) => {
                println!("Non-numeric value input. Game terminated.");
                break;
            }
        };

        if table.is_taken(pos) {
            println!("Position is already taken. Try again.");
            continue;
        }

        table.update(pos, &player);
        println!("");
        table.display();

        if table.is_win(&player) {
            println!("Congratulations, {}, you've won!", player.to_char());
            break;
        }

        if !table.is_tie() {
            println!("Oops, it's a tie! Better luck next time.");
            break;
        }

        player.next_player();
    }
}
