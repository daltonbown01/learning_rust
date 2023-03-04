// Certain libraries are included in all programs. These libraries are called the prelude.
// If we want to use a library that isn't included in the prelude, we need to explicitly bring it into scope with the use statement
// To get quick access to library docs, run cargo doc --open
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// By default not specifying a return value on this function implies this function returns nothing
fn main() {
    // Functions vs macros: Macros look like functions but when called have ! at the end
    println!("Guess the number!");

    // Let defines a variable
    // No need for mut here since we don't want to reassign this variable later on
    // Range is inclusive on upper and lower bounds
    // Rust defaults numbers to i32 when inferring number types
    // = is called binding (i.e., we're binding the randomly generated number to the secret_num variable on this line)
    let secret_num = rand::thread_rng().gen_range(1..=100);

    // Commenting this out to not ruin the fun of the game
    // println!("The secret number is {secret_num}!");

    // Creates an infinite loop
    loop {
        println!("Please input your guess.");

        // mut because we need to reassign guess later
        // String::new() creates an empty, growable string obj
        let mut guess = String::new();

        // Best practice when chaining > 1 .method is to split them into new lines
        // & indicates we reference a variable, allowing us to access a variable multiple times
        // We need to specify &mut since we want read_line to be able to modify the value of guess
        // .read_line returns a Result enum which is either Ok or Err
        // If an error occurred, Err will be set and will contain data related to the error
        // If the operation was successful, Ok will contain the return value from the method
        // .expect will crash the program if Err, and will print what we pass to that method
        // If no error occurred, .expect will just return the value stored in Ok
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess is called shadowing which lets us reuse a variable instead of defining a new unique one such as guess_str and guess
        // Shadowing is useful when converting a value from one type to another
        // Trim is necessary as the read_line method requires the user to click enter, which adds a \n character
        // .parse knows what type we're trying to parse since we explicitly typed the guess variable with let guess: u32
        // Err(_) is a catch all indicating we want to take that arm on any err values
        // Continue immediately starts another iteration of the loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Compares guess to secret_num
        // Match will selectively execute the code based on the output of the cmp (kinda like a switch case)
        // Each case in the match is called an arm
        // Match checks each arm in order, and will stop once it finds the first successful match
        // Since we're comparing guess which is explicitly typed as u32, secret_num is now inferred as u32 instead of i32
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // Breaks out of the infinite loop
                break;
            }
        }
    }
}
