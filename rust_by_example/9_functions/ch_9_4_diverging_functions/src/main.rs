// #![feature(never_type)]

fn main() {
    // Diverging functions never return. They are marked using !, which is an empty type.

    fn foo() -> ! {
        panic!("This call never returns.");
    }

    // As opposed to all the other types, this one cannot be instantiated,
    // because the set of all possible values this type can have is empty.
    // Note that, it is different from the () type, which has exactly one possible value.
    //
    // For example, this function returns as usual,
    // although there is no information in the return value.

    fn some_fn() {
        ()
    }

    let _a: () = some_fn();
    println!("This function returns and you can see this line.");

    // Ass opposed to this function, which will never return the control back to the caller

    // let x: ! = panic!("This call never returns.");
    // println!("You will never seen this line!");

    // Although this might seem like an abstract concept, it is in fact very useful and often handy.
    // The main advantage of this type is that it can be cast to any other one
    // and therefore used at places where an exact type is required,
    // for instance in match branches. This allows us to write code like this:

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Notice that the return type of this match expression must be u32
            // because of the type of the "addition" variable.
            let addition: u32 = match i % 2 == 1 {
                // The "i" variable is of type u32, which is perfectly fine.
                true => i,
                // On the other hand, the "continue" expression does not return
                // u32, but it is still fine, because it never returns and therefore
                // does not violate the type requirements of the match expression.
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!(
        "Sum of odd numbers up to 9 (excluding): {}",
        sum_odd_numbers(9)
    );

    // ! is also the return type of functions that loop forever (e.g. loop {}) like network
    // server or functions that terminate the process (e.g. exit() ).
}
