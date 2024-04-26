// This structure cannot be printed either with 'fmt::Display' or with 'fmt::Debug'
#[allow(dead_code)]
struct UnPrintable(i32);

//derive attribute automatically creates the implementation required to make this 'struct'
//printable with 'fmt::Debug'
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // Print with {:?} is similar to with {}
    println!("{:?} months in a year. ", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );
    // Standard fmt::Debug print
    println!("Now {:?}, will print!", Deep(Structure(7)));

    // Pretty print
    println!("Now {:#?}, will print!", Deep(Structure(7)));
}
