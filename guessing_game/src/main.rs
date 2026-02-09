use rand::{self, RngExt};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the number guessing game!");

    // Generate a random integer between 0 and 100
    // Note: I use `..=` so that the range is inclusive.
    //       using `..` instead would give 0 > 99.

    let mut rng = rand::rng();
    let secret: u32 = rng.random_range(0..=100);

    println!("The secret is: {}", secret);

    println!("Please enter a number between 0 and 100");

    // Creating a guess variable to get the input.
    // In rust we need to always handle all cases
    // --> we use expect to handle errors.

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("The input was not received correctly");

    // We need to parse the input to an number to compare

    let guess: u32 = guess
        .trim()
        .parse()
        .expect("Unable to parse the given input to a number");

    println!("Your guess: {}", guess);

    match guess.cmp(&secret) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("PERFECT! Congratulations :D"),
    }
}
