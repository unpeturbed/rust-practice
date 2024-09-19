// Modify the program to take user input and print "Hello, [name]!".
use std::io::{self};
fn main() {
    
    let mut name = String::new();
    println!("Please enter your name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Unable to read user input");

    //Trim the newline from the input    
    println!("Hello, {}", name.trim());
}
