use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the number!");
    println!("Enter your guess.");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {guess}");
    println!("The secret number is {secret_number}");
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("To big!"),
        Ordering::Equal => println!("You win!"),
    }
}
