// Create a program that increments a counter variable each time a button is pressed.

fn main() {
    let mut counter = 0;

    for _ in 0..10 {
        counter += 1;
    }

    println!("Counter = {}", counter);
}
