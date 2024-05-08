// With let-else, a refutable pattern can match and bind variables
// in the surrounding scope like a normal let,
// or else diverge (e.g. break, return, panic!) when the pattern doesn't match.

use std::str::FromStr;

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(" ");
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}' ");
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: '{count_str}' ");
    };
    (count, item)
}

fn main() {
    // LET ELSE decision...
    println!("LET ELSE decision...");
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));

    // CLASSIC decision...
    println!("CLASSIC decision...");
    let s = "3 chairs";
    let mut it = s.split(" ");
    let (count_str, item) = match (it.next(), it.next()) {
        (Some(count_str), Some(item)) => (count_str, item),
        _ => panic!("Can't segment count item pair: '{s}'"),
    };
    let count = if let Ok(count) = u64::from_str(count_str) {
        count
    } else {
        panic!("Can't parse integer: '{count_str}'");
    };

    assert_eq!((count, item), (3, "chairs"));
}
