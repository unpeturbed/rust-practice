// Convert a string to an integer and print the result.

fn main() {

    let num_str: &str = "10";
    let num: i32 = num_str.parse().unwrap();

    println!("num = {num}");
}

