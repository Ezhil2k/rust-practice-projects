//Guessing Game
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100000);
    // let secret_number = 74;
    let mut p = 1;
    let mut b = 100000;
    
    loop {
        let mid = (p + b) / 2;
        println!("Please input your guess.");

        let guess = mid;

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => { p = mid; println!("Too small") },
            Ordering::Greater => { b = mid; println!("Too big") },
            Ordering::Equal => {
                println!("You win!, Number is {}", guess);
                break;
            }
        }
    }
}
