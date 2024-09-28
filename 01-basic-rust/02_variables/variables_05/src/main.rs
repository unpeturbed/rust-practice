// Create a program that calculates the area of a rectangle.

use variables_05::Rectangle;

fn main() {
    let width = -30.6;
    let height = 50.8;

    let rectangle = Rectangle { width, height };
    let area = rectangle.area();

    println!("The area of the rectangle is {area}");
    
}

