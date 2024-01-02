use rand::Rng; //random number generator from rand library
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    //the thread_rng() function gives us the particular random number generator, one that is local to the current thread of execution.
    // the gen_range(start..=end);

    println!("Please input your guess.");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: i32 = guess.trim().parse().expect("PLEASE TYPE A NUMBER");

    println!("You guessed: {guess}");
    println!("The secret number is: {secret_number}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!!"),
        Ordering::Greater => println!("too large!!"),
        Ordering::Equal => println!("YOU WIN!!!!"),
    }
}
