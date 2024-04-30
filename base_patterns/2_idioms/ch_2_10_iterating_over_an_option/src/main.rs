// Option can be viewed as a container that contains either zero or one element.
// In particular, it implements the IntoIterator trait,
// and as such can be used with generic code that needs such a type.

fn main() {
    let turing = Some("Turing");
    let mut logicians = vec!["Curry", "Kleene", "Markov"];
    logicians.extend(turing);
    assert_eq!(Some(&"Turing"), logicians.last());

    //equivalent to
    // if let Some(turing_inner) = turing {
    //     logicians.push(turing_inner)
    // }

    //If you need to tack an Option to the end of an existing iterator,
    // you can pass it to .chain()

    let turing = Some("Turing");
    let logicians = vec!["Curry", "Kleene", "Markov"];

    for logician in logicians.iter().chain(turing.iter()) {
        println!("{logician} is a logician");
    }
    println!("{logicians:#?}");
}
