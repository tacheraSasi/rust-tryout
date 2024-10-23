use std::io;

fn main() {
    println!("-- Guess a number --");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Something went, Failed to read the line");

    
}

