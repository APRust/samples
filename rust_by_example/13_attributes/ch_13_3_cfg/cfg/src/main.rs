// Configuration conditional checks are possible through two different operators:
//
//  * the cfg attribute: #[cfg(...)] in attribute position
//  * the cfg! macro: cfg!(...) in boolean expressions
//
// While the former '#[cfg()]' enables conditional compilation, the latter 'cfg!()'
// conditionally evaluates to true or false literals allowing for checks at run-time.
// Both utilize identical argument syntax.
//
// cfg!, unlike #[cfg], does not remove any code and only evaluates to true or false.
// For example, all blocks in an if/else expression need to be valid
// when cfg! is used for the condition, regardless of what cfg! is evaluating.

// this function only gets compiled of the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS in "not" linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
