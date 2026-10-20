use color_print::cprintln;
use dialoguer::{BasicHistory, Input};
use std::cmp::Ordering;
use std::process;
use std::{io, str};

fn read_number(prompt: &str) -> f64 {
    loop {
        println!("{prompt}");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("could not read input");

        match input.trim().parse::<f64>() {
            Ok(n) => return n,
            Err(_) => println!("not a valid number! please input a real number."),
        }
    }
}

fn main() {
    let first_number = read_number("type in your first number:");
    let second_number = read_number("type in your second number:");

    println!("Guess the number! the number is between {first_number} and {second_number}");
    println!();

    let secret_number: f64 = rand::random_range(first_number..=second_number);

    //println!("the secret number is: {secret_number}");

    let mut history = BasicHistory::new().max_entries(8).no_duplicates(true);

    let mut attempts = 0;

    loop {
        println!("attempt: #{attempts}");
        println!("Please input your guess");

        if let Ok(guess) = Input::<String>::new()
            .history_with(&mut history)
            .interact_text()
        {
            if guess == "exit" {
                process::exit(0);
            }
            println!();
            println!("-----------------");

            //let mut guess = String::new();
            //io::stdin()
            //    .read_line(&mut guess)
            //    .expect("Failed to read line");

            let guess: f64 = match guess.trim().parse() {
                Ok(num) => {
                    attempts += 1;
                    num
                }
                Err(_) => {
                    cprintln!("<green>Thats not a valid number! type in a number.</>");
                    println!("-----------------");
                    continue;
                }
            };

            match guess.total_cmp(&secret_number) {
                Ordering::Less => {
                    cprintln!("<red>Too small!</>");
                    println!("-----------------")
                }
                Ordering::Greater => {
                    cprintln!("<red>Too big!</>");
                    println!("-----------------")
                }
                Ordering::Equal => {
                    println!("You guessed correctly!");
                    println!("number of attempts: {attempts} guesses");
                    break;
                }
            }
        }
    }
}
