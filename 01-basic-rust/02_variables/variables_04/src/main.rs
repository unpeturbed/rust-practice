// Shadow a variable and print both values.

fn main() {
    let input = 5;
    println!("Before shadowed by inner scope, input = {input}");
    {
        let input = "Universe";
        println!("After shadowed by inner scope, input = {input}");
    }
    let input = 45.3;
    println!("After shadowed by outer scope, input = {input}");
}

