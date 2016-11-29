extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Secret number {}", secret_number);

    loop {
        let mut guess = String::new();
        
        println!("Guess a number: ");
        
        io::stdin().read_line(&mut guess).expect("Failed to readline");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
        }
    }
}
