// Format a string with multiple variables and print it.

fn main() {
    let name: String = "Unpeturbed".into();
    let surn_name = String::from("Petur");
    let occupation: &str = "Dev".into();
    let location: &str = "Metaverse";

    println!("My name is {} {}, and I am a {} who hails from the {}", name, surn_name, occupation, location);
} 

