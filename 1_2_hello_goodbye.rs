// a rust program that takes a number 1 or 2, and prints hello or goodbye respectively. use the match keyword. also, have the number be input as i32, not string matching. use .parse()

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();  

    let trimmed = input.trim();                 
    let number: i32 = trimmed.parse().unwrap(); 


    match number { 
        1 => println!("hello"), 
        2 => println!("goodbye"), 
        _ => println!("unknown option"),
    }
} 
// get the input
// remove newline
// ???
// profit
