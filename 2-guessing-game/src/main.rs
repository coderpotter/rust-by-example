use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100); // inclusive range 1 to 100
    loop {
        println!("Input your guess.");
        let mut guess = String::new(); // mut is for mutable
        io::stdin()
            .read_line(&mut guess) // &mut guess is a reference to guess
            .expect("Failed to read line"); // expect is a method of io::Result
                                            // Quit program if guess is "quit".
        match guess.to_lowercase().trim() {
            "quit" => break,
            _ => (),
        }
        let guess: u32 = match guess.trim().parse() {
            // trim removes whitespace, parse converts string to number.
            // `parse` returns a `Result` type and `Result` is an enum that has the variants `Ok` and `Err`.
            Ok(num) => num, // If `parse` is able to successfully turn the string into a number, it will return an `Ok` value that contains the resulting number.
            // That `Ok` value will match the first arm’s pattern, and the match expression will just return the num value that parse produced and put inside the `Ok` value.
            Err(_) => continue, // If it’s not able to turn the string into a number, it will return an `Err` value that contains more information about the error.
                                // `_` is a catchall value, because we don’t care about the exact error, but we do want to call continue to go to the next iteration of the loop.
        };
        match guess.cmp(&secret_number) {
            // cmp is a method of u32 to compare guess and secret_number
            Ordering::Less => println!("Too small!"), // Ordering is an enum with variants Less, Greater, and Equal
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // break out of loop when guess is correct
            }
        }
    }
}
