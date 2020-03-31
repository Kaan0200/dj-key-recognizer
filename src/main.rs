mod DJKey;
use crate::DJKey::DJKey::CamelotKey;
use std::env;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    // first argument is switch to indicate what type of key we are inputting
    // 1. --camelot -c
    // 2. --key -k
    let switch = &args[1];
    // second argument is the key value to parse
    // either of the form
    // 1. [1-12]{AB}
    // 2. [A-G]{#b}
    let key = &args[2];

    println!("first argument is {}", switch);
    println!("second arg is {}", key);

    let key = key.parse::<CamelotKey>();
}
