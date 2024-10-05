use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess a number between 1 and 10!");

    println!("Input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("You should type in a number!");

    println!("You guessed: {guess}");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("Secret number is {secret_number}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Your guess was too small"),
        Ordering::Greater => println!("Your guess was too big"),
        Ordering::Equal => println!("Your guess was correct"),
    }
}
