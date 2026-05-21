use std::io;
use rand::Rng;
fn main() {


    let mut guess = String::new();
    let apples = 5;
    let secret_number = rand::thread_rng().gen_range(1..=3);
    // we need to specify 1..=100 to request a number between 1 and 100 
    //to include the last number you can use the equal sign

    println!("Hello, world!");
    println!("guess the number");
    
    let x = 5;
    let y = 10;

    println!("x = {x} and y + 2 = {}", y + 2);

    //guesssing game
    println!("Guess the number");
    println!("the secret number is {secret_number}");
    println!("Please input your guess");

    io::stdin()
    .read_line(&mut guess)
    .expect("Faild to read line");

    println!("You guessed: {guess}")

    }
