///Rust does not provide the equivalent to finally blocks -
/// code that will be executed no matter how a function is exited.
/// Instead, an object’s destructor can be used to run code
/// that must be run before exit.

fn main() {
    bar();
}

fn bar() -> Result<(), ()> {
    struct Foo;

    // Implement a destructor for Foo
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("exit");
        }
    }
    // The destructor of _exit will run however the function is exited
    let _exit = Foo;
    // baz()?;
    Err(())?;
    Ok(())
}
