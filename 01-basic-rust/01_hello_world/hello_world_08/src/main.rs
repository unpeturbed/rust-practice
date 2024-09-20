// Print the result of basic arithmetic operations.

fn main() {
    let x: i32 = 41;
    let y: i32 = 5;
    
    let sum_of_numbers = x + y;
    let diff_in_numbers = x - y;
    let product_of_numbers = x * y;
    let division = x as f32 / y as f32;

    println!("{} + {} = {}", x, y, sum_of_numbers); 
    println!("{} - {} = {}", x, y, diff_in_numbers); 
    println!("{} x {} = {}", x, y, product_of_numbers); 
    println!("{} / {} = {}", x, y, division); 
}

