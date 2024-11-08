fn add_doubles(closure: fn(i32) -> i32, one: i32, two: i32) -> i32 {
    closure(one) + closure(two)
}

fn add_doubles_with_dyn(closure: Box<dyn Fn(i32) -> i32>, one: i32, two: i32) -> i32 {
    return closure(one) + closure(two);
}

fn main() {
    let test_closure = |string_input| println!("{}", string_input);
    test_closure("test");

    let closure = |int_input| return int_input * 2;

    let outcome = add_doubles(closure, 2, 3);
    println!("{}", outcome);

    // In the preceding code, we can see that we define a closure that doubles
    // an integer that is passed in and returned. We then pass this into
    // our add_doubles function with the notation of fn(i32)-> i32,
    // which is known as a function pointer. When it comes to closures,
    // we can implement one of the following traits:
    // • Fn: Immutably borrows variables
    // • FnMut: Mutably borrows variables
    // • FnOnce: Takes ownership of variables so it can only be called once

    let one = 2;
    let closure = move |int_input| return int_input * one;
    let outcome = add_doubles_with_dyn(Box::new(closure), 2, 3);
    println!("{}", outcome);
}
