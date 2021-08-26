use std::io;
use std::io::*;
use rand::Rng;

fn main() {
    println!("Hello, welcome too rock paper scissors!");
    rules();

    println!("ROCK! PAPER! SCISSORS! SHOOT!");
    result(ai_choice(),user_choice());
}

fn rules(){
    println!("--------------------------------------------------\nRULES\n--------------------------------------------------\nEnter: \n\t R = Rock \n\t P = Paper \n\t S = Scissors\n-------------------------------------------------- \nThe computer will reveal its choice and the winner");
}

fn user_input()-> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    return input
}

fn user_choice()->u8{
    //defines variables
    let mut input = user_input();
    let mut choice:u8 = 3;



    //if statement too convert the user choice into the same data type as ai
    if input == "r" {
        choice = 0;
    }
    else if  input == "p"{
        choice = 1;
    }
    else if  input == "s"{
        choice = 2;
    }
    else{
        println!("Error");
    }
    println!("{},{}",choice,input);
    //returns choice
    return choice
}

fn ai_choice()->u8{
    //specify thread
    let mut rng = rand::thread_rng();
    //random number
    let choice: u8 = rng.gen_range(0..2);
    return choice
}

fn ai_output(ai_choice:u8){
    match ai_choice {
        0 => println!("Computer chose Rock"),
        1 => println!("Computer chose Paper"),
        2 => println!("Computer chose Scissors"),
        other => println!("Error with choice number {}",other),
    }

}

fn result(ai_choice: u8 , user_choice: u8) {
    ai_output(ai_choice);
    let mut win:u8 = 0;

    if user_choice == ai_choice {
        win = 3;
    }
    //>>>>>USER CHOOSES ROCK<<<<<//
    else if user_choice == 0{
        if ai_choice == 1{
            win = 1;
        }
        else if ai_choice == 2{
            win = 2;
        }
    }
    //>>>>>USER CHOOSES PAPER<<<<<//
    else if user_choice == 1{
        if ai_choice == 0{
            win = 2;
        }
        else if ai_choice == 2{
            win = 1;
        }
    }
    //>>>>>USER CHOOSES SCISSORS<<<<<//
    else if user_choice == 2{
        if ai_choice == 0{
            win = 1;
        }
        else if ai_choice == 1{
            win = 2;
        }
    }

    println!("{},{}",ai_choice,user_choice);

    match win{
        1 => println!("Well Done you have won!"),
        2 => println!("Better luck next time you lost :("),
        3 => println!("No winner, no loser but just a tie"),
        other => println!("Error with number {}",other),
    }

}


