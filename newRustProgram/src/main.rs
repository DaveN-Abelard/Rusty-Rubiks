use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    //This prints the title of the program
    println!("Guess the number!");




    let secret = rand::thread_rng().gen_range(1..100);

    loop {
        //This prints the the prompt
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number.");
                continue;
            }
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret) {
            Ordering::Less => println!("too small :("),
            Ordering::Equal => {
                println!("correct! :)");
                break;
            },
            Ordering::Greater => println!("too big :("),
        }
    }
}