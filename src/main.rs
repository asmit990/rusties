use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Guess the number!");

        let mut guess = String::new();

        // Handle the result of read_line
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("failed to parse!");
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("boi dont type your penis size"),
            Ordering::Equal => println!("You won"),
            Ordering::Greater => println!("too big!"),
        }
    }
}
