use core::num;
use std::thread::LocalKey;
use std::{io, string};
use std::cmp::Ordering;
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


loop {

    let mut amount  = String::new();

    println!("your available balance is {balance}");
    io::stdin().read_line(&mut amount).expect("system crashed");

    //Rust hates crashes. Rust wants to know: "What is the backup plan if the user types garbage?"
    //Inside that gift box, there are only two possibilities:
    //Ok(num) -> "Everything went great, here is the actual number!"
    //"I opened the box and it's an Ok! It contains a valid number (which we will temporarily call num). Because this matched, I am going to pass that num out of the match game and save it into our final amount variable."

    //Err(_) -> "Uh oh, they typed letters. This is an error."
    //"I opened the box and it's an Err (an error). The user typed something stupid like 'abc'. The _ means 'I don't care what kind of error it is'. Instead of saving a number, I am hitting the continue button. This instantly stops what we are doing, ignores the rest of the code below, and jumps right back to the top of the loop to ask the user again."
     
let amount: i32 = match amount.trim().parse() {
    Ok(num) => num,
    Err(_) => continue, 
};


   match amount.cmp(&balance){
    Ordering::Equal => {
        println!("procesing");
        println!("here is your cash, your remaining balance is 0");
        println!("thanks for banking with us, have a nice day sir");
        break;


    }
    Ordering::Less => {
        println!("procesing");
        balance = balance - amount ;

        //Because you used let, you accidentally shadowed the balance only inside those curly braces { }. You created a temporary, brand-new balance variable that disappears the moment that Less block ends.
        //But imagine if you didn't break the loop, and you let the user make another withdrawal. Because you didn't actually change the original mut balance = 500, their bank account would magically reset back to 500! Free money! 🤑

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

let maxlength = 12 ;
let minlength = 1 ;

//challenge 2
loop {
    let mut  username = String::new();
    println!("please input your new username");

    io::stdin().read_line(&mut username).expect("erro");

    //Think of .expect() as an "automatic bomb." If something goes wrong, it immediately crashes the program with your error message.match is a game where you have to look at every possible outcome (Ok or Err) and provide arms (=>) for themWhen you write match username.trim().expect("error"), you are telling the computer to explode if there's an error, but then you don't provide any match arms for what happens if it succeeds! A match statement must have curly braces {} and arms.

    let username = username.trim();
    let username = username.len();

    if username >= minlength {

        if username <= maxlength {
            println!("approved");
            break;
            
        }

        if username > maxlength {

        println!("username cannot be more than 12 characters");
        }   

    }


       else {
           println!("username cannot be empty");
       }



}







}

