use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret: u32 = rand::thread_rng().gen_range(0..=10);

    loop {
        let mut guess: String = String::new();

        println!("Input your guess.");
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {num},
            Err(_) => {
                if guess.trim().eq("quit") || guess.trim().eq("exit") {
                    break;
                }
                continue;
            },
        };
    
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
        }
    }
}
