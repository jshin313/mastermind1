use std::io::{self, Write};

fn main() {
    println!("This game involves guessing a randomly-generated 6-digit number.");
    print!("Input your guess: ");
    io::stdout().flush().unwrap(); // Stdout has a buffer, so this the above print won't display without flushing

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read stdin");

    println!("Your guess: {}", input);
}
