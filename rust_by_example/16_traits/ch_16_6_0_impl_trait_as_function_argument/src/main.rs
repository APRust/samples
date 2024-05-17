use std::io;
// ` impl Trait ` can be used in two locations:
//
//  1) as an argument type
//  2) as a return type
//
// ....
//
// AS AN ARGUMENT TYPE
// If your function is generic over a trait but you don't mind the specific type,
// you can simplify the function declaration using ` impl Trait ` as the type of the argument
//
// For example, consider the following code:
fn parse_csv_document<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // For each line in the source
            line.map(|line| {
                // If the line was read successfully, process it, it not, return error
                line.split(',') // Split the line separated by commas
                    .map(|entry| String::from(entry.trim())) // Remove leading ang trailing whitespace
                    .collect() // Collect all strings in a row into a Vec<String>
            })
        })
        .collect() // Collect all lines into a Vec<Vec<String>>
}
// ` parse_csv_document ` is generic, allowing it to take any type which implements BufRead,
// such as BufReaders<File> or [u8], but it's not important what type `R` is, and `R` is
// only used to declare the type of src, so the function can also be written as:

fn parse_csv_document_impl(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            line.map(|line| {
                line.split(',')
                    .map(|entry| String::from(entry.trim()))
                    .collect()
            })
        })
        .collect()
}
// Note that using `impl Trait` as an argument type means that you cannot explicitly state
// what form of the function you use, i.e. parse_csv_document::<std::io::Empty>(std::io::empty())
// will not work with the second example

fn main() {
    // From Cursor...
    let cursor = io::Cursor::new(b"lorem\nipsum\r\ndolor");
    let from_cursor_one = parse_csv_document(cursor.clone());
    println!("from `cursor`, first: {:?}", from_cursor_one);
    let from_cursor_two = parse_csv_document_impl(cursor.clone());
    println!("from `cursor`, second: {:?}", from_cursor_two);

    // From Empty...
    let from_empty_first = parse_csv_document(std::io::empty());
    println!("from 'empty' first: {:?}", from_empty_first);
    let from_empty_second = parse_csv_document_impl(std::io::empty());
    println!("from 'empty' second: {:?}", from_empty_second);

    // With generic `fish syntax`
    let generic_syntax_first = parse_csv_document::<std::io::Empty>(std::io::empty());

    // With ` impl Trait ` fish-syntax for caller is blocked
    // let generic_syntax_second = parse_csv_document_impl::<std::io::Empty>(std::io::empty());


}
