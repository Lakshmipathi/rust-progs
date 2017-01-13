// Accept value from standard input.

use std::io; // imports io library from standard library

fn main() {
    let mut val = String::new(); //creates new, empty  String

    io::stdin().read_line(&mut val)
             .ok()
             .expect("Failed to read value"); 

    println!("Entered value is: {} ",val);
}
