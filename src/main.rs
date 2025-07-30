use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("🎯 Welcome to the Number Guessing Game!");
    println!("Guess the number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("\nEnter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️ Please enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("🔻 Too low!"),
            Ordering::Greater => println!("🔺 Too high!"),
            Ordering::Equal => {
                println!("🎉 You got it! The number was {}.", secret_number);
                break;
            }
        }
    }
}
