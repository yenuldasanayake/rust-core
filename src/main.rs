use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {


    let mut guess = String::new();
    let apples = 5;
    let secret_number = rand::thread_rng().gen_range(1..=100);
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

    println!("You guessed: {guess}");

    let guess : u32 = guess.trim().parse().expect("Please type a number bro");

    //We use a match expression to decide what to do next based on which variant of Ordering was returned from the call to cmp with the values in guess and secret_number

    //The cmp method compares
//two values and can be called on anything that can be compared. It takes a reference to whatever you
//want to compare with: here it’s comparing guess to secret_number .

//here it’s comparing guess to secret_number . Then it returns a variant of the
//Ordering enum we brought into scope with the use statement

    match guess.cmp(&secret_number){

    Ordering::Less => println!("Too small"), //=> is not a mathematical operater bro
    Ordering::Greater => println!("Too big"),
    Ordering::Equal => println!("You win"),
    }

    //rust allow us to shadow a variable
    //The trim method on a String instance will eliminate any whitespace at the beginning and end,
    //The parse method on strings converts a string to another type. Here, we use it to convert from a string to a number
    // we can convert the string to a u32, (watch the up)
}
