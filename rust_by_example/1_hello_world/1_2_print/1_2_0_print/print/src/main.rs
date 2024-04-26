/*
* format! : write formatted text to String
* print! : same as format! but the text is printed to the console (io::stdout)
* println! : same as print! but a newline is appended
* eprint!: same as print! but the text is printed to the standart error (io::stderr)
* eprintln! : same as eprint! but a newline is appended
*/

fn main() {
    // standard replaced for '{}'
    println!("{} days", 31);

    // positional arguments by numbers
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // named arguments
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    //different formatting for numeric with format character after ':'
    println!("Base 10:                        {}", 56999);
    println!("Base 2 (binary):                {:b}", 56999);
    println!("Base 8 (octal):                 {:o}", 56999);
    println!("Base 16 (hexadecimal):          {:x}", 56999);

    //output "    1". Four whitespace and a "1", for a total width of 5
    println!("{number:>5}", number = 1);
    //right/left padding numbers with extra zeroes or any one chars
    println!("{number:0>5}", number = 1);
    println!("{number:t<5}", number = 9);
    //use named arguments in the format specifier by appending a '$'
    println!("{number:0>width$}", number = 1, width = 5);
    println!("{number:w<width$}", number = 1, width = 5);

    //Only types that implement fmt::Display can by formatted with '{}'.
    //User-defined types do not implement fmt::Display by default

    #[allow(dead_code)] // disable 'dead_code' which warn against unused module
    struct Structure(i32);

    //This wil not compile because 'Structure' does not implement fmt::Display.
    // println!("This struct '{}' won't print...", Structure(3));

    // For Rust 1.58 and above, you can directly capture the argument from a surrounding variable
    // this will output "    1", 4 white space and "1".
    let number: char = 'A';
    let width: usize = 9; // works only for 'usize'
    println!("{number:>width$}");

    // std::fmt contains many traits which govern the display of text.
    // The base form of two important ones are listed below:
    //
    //    fmt::Debug: Uses the {:?} marker. Format text for debugging purposes.
    //    fmt::Display: Uses the {} marker. Format text in a more elegant, user friendly fashion.
    //
    // Here, we used fmt::Display because the std library provides implementations for these types.
    // To print text for custom types, more steps are required.
    // Implementing the fmt::Display trait automatically implements the ToString trait
    // which allows us to convert the type to String.
}
