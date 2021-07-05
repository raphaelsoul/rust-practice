extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess a number!");

    let mut rng = rand::thread_rng();
    let secret = rng.gen_range(0..101);

    loop {
        println!("Input Your Guess:");

        let mut guess = String::from("");
        io::stdin().read_line(&mut guess).expect("Fail read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}
