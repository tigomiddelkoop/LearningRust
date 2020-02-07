use std::io;
// The Input/Output library
use rand::Rng;
use std::cmp::Ordering;

fn main() { // Start the program
    println!("Guess the Number!"); // Print a string to the console

    let secret_number = rand::thread_rng().gen_range(1, 101); // immutable variable

    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess."); // Print another string to the console

        let mut guess = String::new(); // Declare a variable. and set its type to String

        io::stdin().read_line(&mut guess) // Use io:stdin library to read the input of the user
            .expect("Failed to read line"); // When something goes wrong crash and display this value otherwise expect would use the returned value if io::Result returns Ok.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            },
        }; // Transform it to a Integer of 32-bits I assume. While we check if it a number.
        println!("You guessed {}", guess); // print out a line and attach the variable to it.

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too Big!"),
        };
    };
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

/*

    The ampersand(&) in `io::stdin().read_line(&mut guess)` indicates that it is a reference.
    which allows gives the code a way to let the code access a variable without needing to copy into memory multiple times.

    References are immutable by default as well.
    `io::stdin().read_line(&guess)` // immutable
    ``io::stdin().read_line(&mut guess)`` // mutable

*/

/*
    io::stdin().read_line(&mut guess) // Use io:stdin library to read the input of the user
        .expect("Failed to read line"); // If the user fails to input a string error.
    The result instance has an .expect function if it returns "Err" it will crash and use the arguments given to it, if it returns "Ok" it will use the returned values

*/

/*
    println!("You guessed {}", guess); // print out a line and attach the variable to it.

    at last we print out the value, the {} are a placeholder for a value, which you need to give as arguments. (Just like Java)
    You can have more placeholders in a string. The first set of curly brackets holds the first value after the format string
*/

// loop {}, Tells you what it does.