use core::num;
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
    
    let x = 5;
    let y = 10;

    println!("x = {x} and y + 2 = {}", y + 2);

   
   
    println!("Guess the number");
    println!("the secret number is {secret_number}");
   
  
//If parse is able to successfully turn the string into a number, it will return an Ok value that contains the resultant number. That Ok value will match the first arm’s pattern, and the match expression will just return the num value that parse produced and put inside the Ok value.
loop {
     println!("Pleaseinput your guess");


   io::stdin()
    .read_line(&mut guess)
    .expect("Faild to read line");

    let guess : u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue, //it can't execute outside a loop

    };

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number){

    Ordering::Less => println!("Too small"), //=> is not a mathematical operater bro
    Ordering::Greater => println!("Too big"),
    Ordering::Equal => {println!("You win!");
                        break;}
    

}
}
}

