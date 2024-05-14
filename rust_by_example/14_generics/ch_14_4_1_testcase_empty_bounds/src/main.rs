// A consequence of how bounds work is that even if a 'trait' doesn't include
// any functionality, you can still use its as a bound.
// 'Eq' and 'Copy' are example of such trait's from std library.

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// These functions are only valid for types which implement these traits.
// The fact that the traits are empty is irrelevant.
fn red<T: Red>(_: &T) -> &'static str {
    "red"
}

fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    // 'red()' won't work on a blue nor vice versa because of the bounds
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));

    // Error!
    // println!("A turkey is {}", red(&_turkey));
}
