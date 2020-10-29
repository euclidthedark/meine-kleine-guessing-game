use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(0, 100);

    println!("Welcome to our guessing game!");
    println!("Guess a number between 0 and 100.");

    let mut users_guess = String::new();

    io::stdin()
        .read_line(&mut users_guess)
        .expect("Failed to read the line");

    println!("You guessed: {}", users_guess);
    println!("The number we were thinking of was {}.", secret_number);
}
