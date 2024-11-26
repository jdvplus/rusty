use std::io;

fn main() {
    println!("guess the number!");

    println!("please input your guess.");

    let mut guess = String::new(); // create new instance of mutable string 'guess'

    io::stdin()
        .read_line(&mut guess) // append user input to 'guess' (mutable)
        .expect("failed to read line"); // error handling

    println!("you guessed {}", guess);
}
