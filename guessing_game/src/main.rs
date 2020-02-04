use std::io; // The Input/Output library

fn main() { // Start the program
    println!("Guess the Number!"); // Print a string to the console

    println!("Please input your guess."); // Print another string to the console

    let mut guess = String::new(); // Declare a variable. and set its type to String

    io::stdin().read_line(&mut guess) // Use io:stdin library to read the input of the user
        .expect("Failed to read line"); // If the user fails to input a string error.

    println!("You guessed {}", guess); // print out a line and attach the variable to it.
}

/*

    Variables are immutable by default.
    To make variables you need to explicitly say it is mutable by typing `mut` after `let`

    Like so
    `let mut variableName = String::new();`

    Quick Summary
    let foo = 5; // immutable
    let mut bar = 5; // mutable

*/
