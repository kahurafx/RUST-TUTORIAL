use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let string_to_number: u32 = "42".parse().expect("Not a number");

    loop {
        println!("Enter your guess: ");

        let tup = (12, 'c', true);
        // destructuring the tuple
        let (x, y, z) = tup;
        // we can access the elements in a tuple by using the period followed by the index of the element.
        let eg = tup.0;

        // arrays have fixed length.
        let a = [1, 2, 3, 4, 5];

        let secret_guess = rand::thread_rng().gen_range(1..=10);
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("There was an error reading the file");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // This is how we handle errors, the underscore is a catch all other types.

        println!("Your guess is: {guess}");
        match guess.cmp(&secret_guess) {
            Ordering::Less => println!("Your guess is smaller"),
            Ordering::Greater => println!("Your guess is bigger"),
            Ordering::Equal => {
                println!("You WIN!!!!!");
                break;
            }
        }
    }
}
