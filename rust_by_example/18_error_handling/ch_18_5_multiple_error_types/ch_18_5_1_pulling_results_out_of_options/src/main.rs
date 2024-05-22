// The most basic way of handling mixed error types is to just embed
// them in each other:

use std::num::ParseIntError;

fn double_first_one(vec: &Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
}

fn main() {
    // - Option<Result<i32, ParseIntError>>

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("\nExample with 'Option<Result<i32, ParseIntError>...");
    println!("The first doubled is {:?}", double_first_one(&numbers));
    println!("The first doubled is {:?}", double_first_one(&empty));
    println!("The first doubled is {:?}", double_first_one(&strings));

    
    // - Result<Option<i32>, ParseIntError>

    println!("\nExample with 'Result<Option<i32>, ParseIntError>...");
    println!("The first doubled is {:?}", double_first_two(&numbers));
    println!("The first doubled is {:?}", double_first_two(&empty));
    println!("The first doubled is {:?}", double_first_two(&strings));
    
}

fn double_first_two(vec: &Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| n * 2));
    opt.map_or(Ok(None), |r| r.map(|x| Some(x))) // |r| r.map(Some)
}
