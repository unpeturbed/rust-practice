// Create a mutable variable and change its value inside a loop.

fn main() {
    let mut number = 5;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("KABOOOOM!");
}

