use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Secret number is {}", secret_number);

    println!("Enter your number");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Reading the string failed");

    let guess: u32 = guess.trim().parse()
        .expect("Please, enter the number");

    println!("You was guess: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Secret number is bigger than your"),
        Ordering::Greater => println!("Secret number is smaller than your"),
        Ordering::Equal => println!("You win!"),
    }
}
