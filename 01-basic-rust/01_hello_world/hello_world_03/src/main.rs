// Create a program that prints a greeting based on the time of day.
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Get the current time
    let duration_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("Maybe time went backwards");

    let current_hr = (duration_since_epoch.as_secs() /3600) % 24;

    //let current_min = (duration_since_epoch.as_secs() / 60) % 60;
    //println!("{}hr : {}mins", current_hr, current_min);

    match current_hr {
        0..=11 => println!("Good Morning"),
        12..=17 => println!("Good Afternoon"),
        18..=23 => println!("Good Evening"),
        _ => println!("Hello"),
    }
}