use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess: ");

    match parse_guess() {
        Some(number) => println!("You guessed: {}", number),
        None => eprintln!("You didn't enter a number!"),
    }
}

fn parse_guess() -> Option<String> {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    match guess.trim() {
        "" => None,
        _ => Some(guess),
    }
}
