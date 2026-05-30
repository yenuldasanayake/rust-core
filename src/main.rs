use std::io;
use std::cmp::Ordering;
use std::os::unix::io;
use std::ptr::{null, null_mut};
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

//Think of .len() as short for length.
//You create a variable named spaces and put text in it ("   ").
//ou use the let keyword again. Rust says, "Okay, we are destroying the old spaces variable and making a brand new one." Then it counts the length of the old text (which is 3) and saves that number 3 into the new spaces variable.
//git checkout -b your-new-branch-name
// git push -u origin your-branch-name

let mut balance = 500;
let mut amount  = String::new();

loop {

    println!("your available balance is {balance}");
    io::stdin().read_line(&mut amount).expect("system crashed");

    //shadowing but idk wft am i doing now
    let amount :i32 =  amount.trim().parse().expect("please type a number");

   match amount.cmp(&balance){
    Ordering::Equal => {
        println!("procesing");
        println!("here is your cash, your remaining balance is 0");
        println!("thanks for banking with us, have a nice day sir");
        break;


    }
    Ordering::Less => {
        println!("procesing");
        let balance = balance - amount ;
        println!("take your cash");
        println!("your remaining balance is {balance}");
        println!("thanks for banking with us, have a nice day sir");
        break;
    }
    Ordering::Greater => {
        println!("insufficent amount, try again")
    }

   }



}







}

