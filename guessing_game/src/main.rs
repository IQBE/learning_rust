use colored::Colorize;
use rand::{self, RngExt};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("{}", "Welcome to the number guessing game!".blue().bold());

    // Generate a random integer between 0 and 100
    // Note: I use `..=` so that the range is inclusive.
    //       using `..` instead would give 0 > 99.

    let mut rng = rand::rng();
    let secret: u32 = rng.random_range(0..=100);

    println!("Please enter a number {}", "between 0 and 100".yellow());

    loop {
        // Creating a guess variable to get the input.
        // In rust we need to always handle all cases
        // --> we use expect to handle errors.

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("The input was not received correctly");

        let guess = guess.to_ascii_lowercase();

        // if guess.trim() == "quit" || guess.trim() == "q" {
        if ["quit", "q"].contains(&guess.trim()) {
            println!("{}", "Quiting program...".red());
            return;
        }

        // We need to parse the input to an number to compare
        // To do this, we use a match statement so that on error
        // the program doesn't panic.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Invalid input! Try again.".red());
                continue;
            }
        };

        // Comparing is done by the cmp crate. When the number
        // is guessed, break out of the main gameloop.

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small! Try again."),
            Ordering::Greater => println!("Too big! Try again."),
            Ordering::Equal => {
                println!("{}", "PERFECT! Congratulations :D".green().bold());
                return;
            }
        }
    }
}
