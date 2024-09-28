// Parse a string as a number and perform arithmetic on it.

fn main() {

    let num_str: &str = "10";
    let num: i32 = num_str.parse().unwrap();
    println!("{}", num + 5);
}

