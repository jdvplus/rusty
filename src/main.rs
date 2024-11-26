use std::io;
use rand::Rng;

fn main() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("the secret number is {secret number}");

    println!("please input your guess.");

    let mut guess = String::new(); // create new instance of mutable string 'guess'

    io::stdin()
        .read_line(&mut guess) // append user input to 'guess' (mutable)
        .expect("failed to read line"); // error handling

    println!("you guessed {guess}");
}
