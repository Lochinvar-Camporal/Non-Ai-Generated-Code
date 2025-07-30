// write a function named describe_number that: takes an i32, returns a &str, uses a match to return "zero" is the number is 0, "positive" is greater than 0, "negative" if less than 0.
fn describe_number(x: i32) -> &str {
    match x {
        0 => "zero", 
        x if x > 0 => "positive", 
        _ => "negative",
    }
} // i think i get it. its like... if x > 0 x = "positive",
