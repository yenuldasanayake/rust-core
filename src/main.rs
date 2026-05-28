use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("the secret number is {secret_number}");
   
   
   loop {

     println!("Pleaseinput your guess");
     let mut guess = String::new() ;
        


   io::stdin()
    .read_line(&mut guess)
    .expect("Faild to read line");

    let guess : u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue, //it can't execute outside a loop

    };

     println!("you guessed {guess}")
;
   

    match guess.cmp(&secret_number){

    Ordering::Less => println!("Too small"), //=> is not a mathematical operater bro
    Ordering::Greater => println!("Too big"),
    Ordering::Equal => {println!("You win!");
                        break;}

}
}

let mut  x = 5;
println!("the value of x is {x}");
x = 6;
println!("the value of x is {x}");

//const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//shadowing
//When that scope is over, the inner shadowing ends and x returns to being 8 .

let y = 7;
let y = y + 1 ;
//scope
{
    let y = y*15;
    println!("the value of the inner svope is: {y}");
}
println!("the value of y is {y}");
//we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword.


}

