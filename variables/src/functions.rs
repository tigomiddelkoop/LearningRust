/*

   When a rust program boots it will start with the main function
   Rust also uses snake_case as the conventional style for functions and variable names.

   SNAKECASE: all letters are lowercase and "spaces" are underscores.

  Function definitions in rust start with "fn"

*/

fn main() {
    println!("Hello world");
    another_function()
}

fn another_function() {
    println!("Another Function")
}