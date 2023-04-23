use std::{cmp::Ordering, io};

use rand::Rng;

use colored::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret number was {}", secret_number);

    loop {
        println!("guess a number");
        let mut guess_input = String::new();
        io::stdin()
            .read_line(&mut guess_input)
            .expect("Failed to read line");

        // let guessed_number: u32 = guess_input.trim().parse().expect("enter a valid number");
        let guessed_number: u32 = match guess_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "enter a valid number".red());
                continue;
            }
        };

        match guessed_number.cmp(&secret_number) {
            Ordering::Less => println!("{}", "too small".red()),
            Ordering::Greater => println!("{}", "too big".red()),
            Ordering::Equal => {
                println!("{}", "you win".green());
                break;
            }
        }
    }
}
