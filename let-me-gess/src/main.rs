use std::io;

fn main() {
    println!("Try to guess the number!");

    println!("Please enter your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line!");

    println!("Your guess is: {guess}");
}
