use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Try to guess the number!");

    let secret = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 1;

    loop {
        println!("Please enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        println!("Your guess is: {guess}");

        match guess.cmp(&secret) {
            Ordering::Less => {
                println!("Too small!");
                attempts += 1;
            },
            Ordering::Greater => {
                println!("Too big!");
                attempts += 1;
            },
            Ordering::Equal => {
                println!("You win!\nYou've spent {attempts} attempts\nBye!");
                break
            }
        }
    }
}
