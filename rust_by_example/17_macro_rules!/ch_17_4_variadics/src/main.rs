// A variadic interface takes an arbitrary number of arguments. For example, println!
// can take an arbitrary number of arguments, as determined by the format string.

// We can extend our calculate! macro from the previous section to be variadic:

macro_rules! calculate {
    // The pattern for a single `eval`
    (eval $e:expr) => {
       {
        let val: usize = $e; // Forc types to be integers
        println!("{} = {}", stringify!{$e}, val)
       }
    };

    // Decompose multiple `eval`s recursively
    (eval $e: expr, $(eval $es: expr), +) => {
        {
            calculate! { eval $e }
            calculate! { $(eval $es),+ }
        }
    };
}

fn main() {
    calculate! {
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 4) + 1
    }
}
