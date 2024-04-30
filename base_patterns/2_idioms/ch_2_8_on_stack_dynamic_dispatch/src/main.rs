use std::fs;
use std::io;

/// We can dynamically dispatch over multiple values, however, to do so,
/// we need to declare multiple variables to bind differently-typed objects.
/// To extend the lifetime as necessary, we can use deferred conditional initialization,
/// as seen below:

fn main() {
    // These must live longer than `readable`, and thus are declared first:
    let (mut stdin_read, mut file_read);

    // We need to describe the type to get dynamic dispatch.
    let readable: &mut dyn io::Read = if arg == "-" {
        stdin_read = io::stdin();
        &mut stdin_read
    } else {
        file_read = fs::File::open(arg)?;
        &mut file_read
    };

    // Read from `readable` here.
}
