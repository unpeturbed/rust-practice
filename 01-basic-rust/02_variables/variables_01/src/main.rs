// Swap two variables without using a third variable.

use variables_01::Point;

fn main() {

    // Create a mutable variable and swap its values.
    let mut number = Point { first_param: 5, second_param: 10 };
    number.swap();

    println!("First = {}\n Second = {}", number.first_param, number.second_param);

    //Create a mutable string and swap its values.

    let mut word = Point { first_param: "hello".to_string(), second_param: "world".to_string() };
    word.swap();

    println!("first = {} second = {}", word.first_param, word.second_param);
}

