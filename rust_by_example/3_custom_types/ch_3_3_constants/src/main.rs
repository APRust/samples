// Rust has two different types of constants which can be declared 
// in any scope including global. 
// Both require explicit type annotation:
// 
// - const: 
// An unchangeable value (the common case).
//
// - static: 
// A possibly mutable variable with 'static lifetime. 
// The static lifetime is inferred and does not have to be specified. 
// Accessing or modifying a mutable static variable is unsafe.


// Global are declared outside all other scopes.
static mut LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;
    
    // Access constant in the main thread
    unsafe {
        println!("This is {}", LANGUAGE);
    }
    
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) {"big"} else { "small"});
    
    // Error! Cannot modify a 'const'.
    unsafe {
        LANGUAGE = "New string";
    }
}
