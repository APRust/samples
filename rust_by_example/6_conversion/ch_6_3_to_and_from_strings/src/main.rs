// To convert any type to a String is as simple as implementing the ToString trait for the type. 
// Rather than doing so directly, you should implement the fmt::Display trait 
// which automagically provides ToString and also allows printing 
// the type as discussed in the section on print!.

use std::fmt;
use std::fmt::Formatter;

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Circle {{ radius: {} }}", self.radius)
    }
}
fn main() {
    let circle = Circle{ radius: 6};
    println!("{}", circle.to_string());

    // PARSING A STRING
    // use the parse function and either to arrange for type inference
    let parsed: i32 = "5".parse().unwrap();

    // specify the type to parse using the 'turbofish' syntax
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {}", sum)
}
