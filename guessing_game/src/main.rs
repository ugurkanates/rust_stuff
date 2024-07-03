use std::io; // Import the input/output library
use rand::Rng; // Import the random number generation library
use std::cmp::Ordering; // Import the comparison ordering enum

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // Generate a random number between 1 and 100

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // Create a mutable string to store the user's guess

        io::stdin()
            .read_line(&mut guess) // Read the user's input and store it in the guess variable
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { // Convert the guess from a string to a u32
            Ok(num) => num,
            Err(_) => continue, // If the input is not a number, continue to the next iteration
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) { // Compare the guess to the secret number
            Ordering::Less => println!("Too small!"), // If the guess is less than the secret number
            Ordering::Greater => println!("Too big!"), // If the guess is greater than the secret number
            Ordering::Equal => {
                println!("You win!"); // If the guess is equal to the secret number
                break; // Exit the loop
            }
        }
    }
}