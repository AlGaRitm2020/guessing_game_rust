use std::io;

fn main() {
    println!("Guess the number!");

    println!("Enter your number");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Reading the string failed");

    println!("You was guess: {}", guess);
}
