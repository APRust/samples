//Import (via 'use') the 'fmt' module to make it available.
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
struct Structure(i32);

//To use the '{}' marker, the trait 'fmt::Display' must be implemented
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "self.0 = {}", self.0)
    }
}

fn main() {
    //Print with Display'{}'
    println!("Display print is: {}", Structure(23));

    //Print with Debug
    println!("Debug print is: {:?}", Structure(23));
    println!("Pretty Debug print is: {:#?}", Structure(23));
}
