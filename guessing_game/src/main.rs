use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please input your guess.");

        let mut guess = String::new(); // create new instance of mutable string 'guess'

        io::stdin()
            .read_line(&mut guess) // append user input to 'guess' (mutable)
            .expect("failed to read line."); // error handling

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("(please enter a valid number next time!)");
                continue;
            }
        }; // need semicolon here because of variable declaration (?)

        println!("you guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("congratulations! you win.");
                break;
            }
        }
    }
}