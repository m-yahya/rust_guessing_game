// loading crates
use rand::Rng;
use std::cmp::Ordering;
use std::io;
// main function

fn main() {
    // generate random number
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    // start loop
    loop {
        // declare a placeholder to store the user input
        let mut guess = String::new();
        println!("Please enter guess!");
        // read the user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // convert the string into integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        // check input
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess is too low!"),
            Ordering::Greater => println!("Guess is too high!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
