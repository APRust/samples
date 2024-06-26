// A way to write simple code while preserving the original errors is to Box them.
// The drawback is that the underlying error type is only known at runtime
// and not statically determined.
//
// The stdlib helps in boxing our errors by having Box implement conversion from
// any type that implements the Error trait into the trait object Box<Error>, via From.

use std::error;
use std::fmt;
use std::fmt::{Display, Formatter};

// Change the alias to use `Box<dyn error::Error>`
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl Display for EmptyVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "invalid first item for double")
    }
}

impl error::Error for EmptyVec {}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into()) // Converts to Box
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into()) // Converts to Box
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => {
            println!("The first doubled is {}", n)
        }
        Err(e) => {
            println!("Error: {}", e)
        }
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
