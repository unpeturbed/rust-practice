// Implement a program that changes a variable's value based on user input.

fn main() {
    let mut value: u32 = 54;

    println!("Original Value: {value}");

    println!("Enter a new number: ");

    let mut input  = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input from user");

    let input: u32 = input
        .trim()
        .parse()
        .expect("Please type a number!");

    value = input;

    println!("You typed: {input}");

    println!("New Value: {value}");
}
