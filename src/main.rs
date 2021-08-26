use std::io;
use std::io::*;
use rand::Rng;

fn main() {
    println!("Hello, welcome too rock paper scissors!");
    rules();

    println!("ROCK! PAPER! SCISSORS! SHOOT!");
    result(aiChoice(),userChoice());
}

fn rules(){
    println!("--------------------------------------------------\nRULES\n--------------------------------------------------\nEnter: \n\t R = Rock \n\t P = Paper \n\t S = Scissors\n-------------------------------------------------- \nThe computer will reveal its choice and the winner");
}

fn userInput()-> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    return input
}

fn userChoice()->u8{
    //defines variables
    let mut input = userInput();
    let mut choice:u8 = 3;

    //sets outcomes to variables
    let r = String::from("r");
    let p = String::from("p");
    let s = String::from("s");

    //switch statment
    match input {
        r => choice = 0,
        p => choice = 1,
        s => choice = 2,
        other => println!("Error {} is invalid", other),
    }
    //returns choice
    return choice
}

fn aiChoice()->u8{
    //specify thread
    let mut rng = rand::thread_rng();
    //random number
    let choice: u8 = rng.gen_range(0..2);
    println!("{}",choice);
    aiOutput(choice);
    return choice
}

fn aiOutput(aiChoice:u8){
    match aiChoice {
        0 => println!("Computer chose Rock"),
        1 => println!("Computer chose Paper"),
        2 => println!("Computer chose Scissors"),
        other => println!("Error with choice number {}",other),
    }

}

fn result(aiChoice: u8 , userChoice: u8) {
    let mut win:u8 = 0;

    if userChoice == aiChoice {
        win = 3;
    }
    //>>>>>USER CHOOSES ROCK<<<<<//
    else if userChoice == 0{
        if aiChoice == 1{
            win = 1;
        }
        else if aiChoice == 2{
            win = 2;
        }
    }
    //>>>>>USER CHOOSES PAPER<<<<<//
    else if userChoice == 1{
        if aiChoice == 0{
            win = 2;
        }
        else if aiChoice == 2{
            win = 1;
        }
    }
    //>>>>>USER CHOOSES SCISSORS<<<<<//
    else if userChoice == 2{
        if aiChoice == 0{
            win = 1;
        }
        else if aiChoice == 1{
            win = 2;
        }
    }

    println!("{},{}",aiChoice,userChoice);

    match win{
        1 => println!("Well Done you have won!"),
        2 => println!("Better luck next time you lost :("),
        3 => println!("No winner, no loser but just a tie"),
        other => println!("Error with number {}",other),
    }

}


