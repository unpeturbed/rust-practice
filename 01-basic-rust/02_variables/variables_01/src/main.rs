// Swap two variables without using a third variable.

use variables_01::Point;

fn main() {

    let mut swapped = Point { x: 5, y: 10 };
    swapped.swap();

    println!("x = {}\n y = {}", swapped.x, swapped.y);
}
