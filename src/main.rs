use std::io::{self, Write};
use rand::Rng;

fn main() {

    // Generates a random number
    let rand_number = rand::thread_rng().gen_range(1, 999999);

    println!("This game involves guessing a randomly-generated 6-digit number.");

    let correct = "This digit is correct and in the right place";
    let sortof = "This digit is in the number, but not in the right place";
    let incorrect = "This digit is not in the number.";

    let mut tries = 0;

    loop {

        // Keeps track of right, almost right, and wrong guesses
        let mut right = 0;
        let mut almost = 0;
        let mut wrong = 0;

        print!("Input your guess: ");
        io::stdout().flush().unwrap(); // Stdout has a buffer, so this the above print won't display without flushing

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read stdin");

        // Get user input
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        tries += 1; // Increments the number of guesses the user put in

        for i in (0..6).rev() {
            // Isolates one digit from each number
            let rand_digit = rand_number / u32::pow(10, i) % 10;
            let input_digit = input / u32::pow(10, i) % 10;

            if rand_digit == input_digit {
                println!("{} <-- {}", input_digit, correct);
                right+=1;
            }
            else
            {
                // Checks if the input digit is at least present in the number
                for j in 0..6 {
                    // Isolates one digit from each number
                    let rand_digit = rand_number / u32::pow(10, j) % 10;
                    if rand_digit == input_digit {
                        println!("{} <-- {}", input_digit, sortof);
                        almost += 1;
                        break;
                    }

                    // If no match found anywhere in the random number
                    if j == 5 {
                        println!("{} <-- {}", input_digit, incorrect);
                        wrong += 1;
                    }
                }

            }

        }

        // Prints how many digits the user got right, almost right, and wrong
        println!("");
        println!("You guessed {} digits correctly in the right position", right);
        println!("You guessed {} digits correctly, but not in the right position", almost);
        println!("You guessed {} digits completely incorrectly (i.e. not in the randomly generated number)", wrong);

        // For debug purposes
        // println!("Random number: {}", rand_number);

        if input == rand_number {
            println!("Congratulations! You guessed the right number. You won!");
            println!("It took you {} tries to guess the right number.", tries);
            break;
        }
    }


}
