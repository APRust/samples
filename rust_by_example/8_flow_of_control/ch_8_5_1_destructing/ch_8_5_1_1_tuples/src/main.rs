fn main() {
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);
    // Match can be used for destructure a tuple
    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("First is '0', 'y' is {:?}, and 'z' is {:?}", y, z),
        (1, ..) => println!("First is '1' and the rest doesn't matter"),
        (.., 2) => println!("last is '2' and the rest doesn't matter"),
        (3, .., 4) => println!("First is '3', last is '4', and the rest doesn't matter"),
        // '_' means don't bind the value to a variable
        _ => println!("It doesn't matter what they are"),
    }
}
