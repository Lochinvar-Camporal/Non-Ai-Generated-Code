// write a program that string matches apple to crunchy and banana to soft. dont actually take input from the user, just hardcode apple as the matched string.
fn main() { 
    let fruit = “apple”;
    match fruit {
        “apple” => println!(“crunchy!”),
        “banana” => println!(“soft”),
        _ => println!(“unknown”)
    }
} 
// ok now what?
