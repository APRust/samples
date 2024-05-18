// A DSL is a mini "language" embedded in a Rust macro. It is completely valid Rust
// because the macro system expands into normal Rust constructs, but it looks
// like a small language. This allows you to define concise or intuitive
// syntax for some special functionality (within bounds).
//
// Suppose that I want to define a little calculator API. I would like to supply
// an expression and have the output printed to console.

macro_rules! calculate {
    (eval $e:expr) => {
        let val: f64 = $e; // Force types to be unsigned integers
        println!("{} = {}", stringify!{$e}, val);
    };
}

fn main() {
    calculate! {
        eval  1.0 + 2.0
    }

    calculate! {
        eval 1.0 + 2.0 * 3.0 / 4.0
    }
}
