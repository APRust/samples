fn main() {
    println!("{}", say_hello("Alex"));
}


fn say_hello(name: &str) -> String {
    //We could construct the result string manually.
    // let mut result = "Hello ".to_owned();
    // result.push_str(name);
    // result.push('!');
    // result

    //But using format! is better.
    format!("Hello {name}!")
}