// write a function called grade_feedback that: takes a i32 representing a test score, returns a &str with feedback based on the score. use a match to return: "perfect" if the score is exactly 100
// "great" is the score is 90 or above
// "pass if the score is 60 or above
// "fail" otherwise
fn grade_feedback(x: i32) -> &str {
    match x {
        100 => "perfect",
        x if x >= 90 => "great",
        x if x >= 60 && x <= 90 => "pass",
        _ => "fail"
    }
} // is this good?
