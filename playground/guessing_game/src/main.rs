use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number : u32 = rand::random_range(1..=100);
    // println!("(debug) The secret number is: {secret_number}");
    let mut attempts : u32 = 0;
    let max_attempts : u32 = 7;
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // let guess : u32 = guess.trim().parse().expect("Please type a number!");
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number. Try again.");
                continue;
            }
        };

        if !(1..=100).contains(&guess) {
            println!("Please guess between 1 and 100.");
            continue;
        }

        attempts += 1;

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win in {attempts} attempts.");
                break;
            }
        }
        if attempts == max_attempts {
            println!("Game Over! The number was {secret_number}.");
            break;
        }

        println!("{} attempts left", max_attempts - attempts);
    }
}
