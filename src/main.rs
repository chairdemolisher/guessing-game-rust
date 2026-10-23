use color_print::cprintln;
use crossterm::cursor::DisableBlinking;
use crossterm::style::{Color, Stylize};
use crossterm::terminal::ClearType::All;
use crossterm::terminal::{self, Clear, EnterAlternateScreen};
use crossterm::{cursor, execute, style::Print};
use dialoguer::{BasicHistory, Confirm, Input};
use indicatif::HumanDuration;
use rust_decimal::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};
use std::time::{Duration, Instant};
use std::{cmp::Ordering, io, process, thread};
use terminal_menu::{button, label, menu, mut_menu, numeric, run};

//fn read_number(prompt: &str) -> f64 {
//    loop {
//        println!("{prompt}");
//
//        let mut input = String::new();
//        io::stdin()
//            .read_line(&mut input)
//            .expect("could not read input");
//
//        match input.trim().replace(',', ".").parse::<f64>() {
//            Ok(n) => return n,
//            Err(_) => println!("not a valid number! please input a real number."),
//        }
//    }
//}

fn draw_timer(sw: Instant) {
    let text = format!("timer: {}", HumanDuration(sw.elapsed()));
    let (width, _height) = terminal::size().expect("failed to get terminal size");
    let text = format!("{:>25}", text);
    execute!(
        io::stdout(),
        cursor::SavePosition,
        cursor::MoveTo(width - 25, 0),
        Print(text.red()),
        cursor::RestorePosition
    )
    .unwrap();
}

fn main() {
    loop {
        let menu = menu(vec![
            label("-- velkomen til gætte spillet! --").colorize(Color::Red),
            label("-- vælg indstillinger --").colorize(Color::Magenta),
            numeric("minimum nummer", 1.0, Some(0.5), Some(0.0), Some(1000000.0)),
            numeric(
                "maximum nummer",
                100.0,
                Some(0.5),
                Some(0.0),
                Some(1000000.0),
            ),
            numeric("max decimal places", 10.0, Some(1.0), Some(0.0), Some(15.0)),
            button("start nyt spil"),
            button("quit"),
        ]);

        run(&menu);
        let mm = mut_menu(&menu);

        match mm.selected_item_name() {
            "quit" => return,
            _ => {}
        }
        if mm.canceled() {
            println!("canceled!");
            return;
        }
        let first_number = mm.numeric_value("minimum nummer");
        let second_number = mm.numeric_value("maximum nummer");
        let decimal_places = mm.numeric_value("max decimal places");
        let secret_number: f64 = rand::random_range(first_number..=second_number);
        let mut history = BasicHistory::new().max_entries(8).no_duplicates(true);
        let mut attempts = 0;
        let sw = Instant::now();

        let secret_number = Decimal::from_f64_retain(secret_number)
            .unwrap()
            .round_dp(decimal_places as u32)
            .to_f64()
            .unwrap();

        execute!(io::stdout(), EnterAlternateScreen).expect("couldnt enter alternate screen");
        execute!(io::stdout(), cursor::MoveTo(0, 0), DisableBlinking)
            .expect("failed to move cursor");

        println!("Guess the number! the number is between {first_number} and {second_number}");
        println!("-----------------");

        //println!("the secret number is: {secret_number}"); // used for testing

        let running = Arc::new(AtomicBool::new(true));
        let running_for_thread = Arc::clone(&running);

        let timer = thread::spawn(move || {
            while running_for_thread.load(AtomicOrdering::Relaxed) {
                draw_timer(sw);
                thread::sleep(Duration::from_secs(1));
            }
        });

        loop {
            println!("attempt: #{}", attempts + 1);
            println!("Please input your guess. Press UP to get previous attempts.");
            draw_timer(sw);

            if let Ok(guess) = Input::<String>::new()
                .history_with(&mut history)
                .interact_text()
            {
                if guess == "exit" {
                    process::exit(0);
                }
                println!();
                println!("-----------------");

                let guess: f64 = match guess.trim().replace(',', ".").parse() {
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
                execute!(
                    io::stdout(),
                    Clear(All),
                    cursor::MoveTo(0, 0),
                    DisableBlinking
                )
                .expect("failed to clear lines");

                match guess.total_cmp(&secret_number) {
                    Ordering::Less => {
                        cprintln!("<green>your guess was: {guess}</>");
                        println!("-----------------");
                        cprintln!("<red>Too small!</>");
                        println!("-----------------");
                    }
                    Ordering::Greater => {
                        cprintln!("<green>your guess was: {guess}</>");
                        println!("-----------------");
                        cprintln!("<red>Too big!</>");
                        println!("-----------------");
                    }
                    Ordering::Equal => {
                        println!("You guessed correctly!");
                        println!("number of attempts: {} guesses", attempts);
                        cprintln!("<green>final time: {}</>", HumanDuration(sw.elapsed()));
                        break;
                    }
                }
            }
        }
        running.store(false, AtomicOrdering::Relaxed);
        timer.join().unwrap();

        let confirmation = Confirm::new()
            .with_prompt("Do you want to continue?")
            .interact()
            .unwrap();

        if confirmation {
            continue;
        } else {
            println!("goodbye!");
            break;
        }
    }
}
