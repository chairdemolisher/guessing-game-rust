use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: f64 = rand::random_range(1.0..=2.0);

    println!("the secret number is: {secret_number}");

    let mut attempts = 0;

    loop {
        println!("attempt: #{attempts}");
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: f64 = match guess.trim().parse() {
            Ok(num) => {
                attempts += 1;
                num
            }
            Err(_) => {
                println!("Thats not a number! type in a number.");
                continue;
            }
        };

        match guess.total_cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed correctly!");
                println!("number of attempts: {attempts}");
                break;
            }
        }
    }
}
