use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 11);
    // println!("your secret number is {secret_number}");
    loop {
        println!("🧙‍♂️ Guess a number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("🧙‍♂️ Failed to read from line!");
        println!("🧙‍♂️ You entered {guess}");
        let guess: u32 = guess.trim().parse().expect("🧙‍♂️ That's not valid number!");
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("🧙‍♂️ Equal, You Win!");
                break;
            }
            Ordering::Less => println!("🧙‍♂️ That's less!"),
            Ordering::Greater => println!("🧙‍♂️ That's greater!"),
        }
    }
}
