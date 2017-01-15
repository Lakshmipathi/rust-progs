// Compute area of circle with float type while accepting value from standard input.

use std::io; // imports io lirrary from standard library
fn main() {
    let mut pi = String::new(); //creates new, empty  String
    let mut r = String::new(); 
    let c: f32;

    println!("Enter pi value:");
    io::stdin().read_line(&mut pi)
             .ok()
             .expect("Failed to read value"); 

    println!("Enter radius:");
    io::stdin().read_line(&mut r)
             .ok()
             .expect("Failed to read value"); 

    //Shadowing lets us to re-use the old name.
    // parse() method on String converts the String into number
    let pi: f32 = pi.trim().parse()
              .ok()
              .expect("Please type a number");

    let r: f32 = r.trim().parse()
              .ok()
              .expect("Please type a number");

    c = pi * r * r;  
    println!("Area  {} * {} is {} ", pi, r, c);
}
