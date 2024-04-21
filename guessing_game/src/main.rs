use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    let number = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Take your guess: ");
        let _ = io::stdout().flush();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Aww, too bad you didn't continue playing...");
                break;
            }
        };

        match guess.cmp(&number) {
            Ordering::Less => println!("You've guessed lower."),
            Ordering::Greater => println!("You've guessed higher."),
            Ordering::Equal => {
                println!("You've guessed correctly! Congrats!");
                break;
            }
        }
    }
}
