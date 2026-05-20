use std::io;

fn main() {


    let mut guess = String::new();
    let apples = 5;

    println!("Hello, world!");
    println!("guess the number");
    
    let x = 5;
    let y = 10;

    println!("x = {x} and y + 2 = {}", y + 2);

    //guesssing game
    println!("Guess the number");
    println!("Please input your guess");

    io::stdin()
    .read_line(&mut guess)
    .expect("Faild to read line");

    println!("You guessed: {guess}")

    }
