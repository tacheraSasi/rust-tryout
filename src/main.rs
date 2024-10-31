extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("-- Guess the number --");

    let secret_number = rand::thread_rng().gen_range(1..20);

    loop {
        println!("Please input your number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Try to parse the guess. If it fails, ask for input again.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,   // If parsing is successful, set `guess` to `num`
            Err(_) => {
                println!("Please enter a valid number!"); 
                continue; // Skip the rest of the loop if parsing fails
            }
        };

        println!("You guessed: {}", guess);

        // Compare guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
