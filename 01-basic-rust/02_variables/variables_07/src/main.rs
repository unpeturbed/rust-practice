// Declare a variable with a tuple and access its elements.

fn main() {
    let numbers = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10);

        
    let first_number = numbers.0;
    let tenth_number = numbers.9;
    println!("The first number is {} and the tenth number is {}", first_number, tenth_number);
}

