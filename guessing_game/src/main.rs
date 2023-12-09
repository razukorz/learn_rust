use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32= match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("This is not a number.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small."),
            Ordering::Equal => {
                println!("Correct!");
                break;
            },
            Ordering::Greater => println!("Too Big.")
        }
    }
}
