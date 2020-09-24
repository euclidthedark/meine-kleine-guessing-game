use std::io;

fn main() {
    println!("Welcome to our guessing game!");
    println!("Guess a number...");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    println!("You guessed: {}", guess);
}
