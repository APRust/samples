// Use cargo doc to build documentation in target/doc, cargo doc --open
// will automatically open it in your web browser.
//
// Use cargo test to run all tests (including documentation tests),
// and cargo test --doc to only run documentation tests.
//
// These commands will appropriately invoke rustdoc (and rustc) as required.
//
//
// DOC COMMENTS
//
// Doc comments are very useful for big projects that require documentation.
// When running rustdoc, these are the comments that get compiled into documentation.
// They are denoted by a ///, and support Markdown.

// #![crate_name = "doc"]

/// A human being is represented here
pub struct Person {
    /// A person must have a name, no matter how much Juliet may hate it
    name: String,
}

impl Person {
    /// Creates a person with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// Gives a friendly hello!
    ///
    /// Says "Hello, [name](Person::name)" to the `Person` it is called on.
    pub fn hello(&self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    let john = Person::new("John");

    john.hello();
}
//
// To run the tests, first build the code as a library, then tell rustdoc where
// to find the library so it can link it into each doctest program:
//
// $ rustc doc.rs --crate-type lib
// $ rustdoc --test --extern doc="libdoc.rlib" doc.rs

// DOC ATTRIBUTES
//
// Below are a few examples of the most common #[doc] attributes used with rustdoc.
//
//  * INLINE
// Used to inline docs, instead of linking out to separate page.
//

#[doc(inline)]
pub use bar::Bar;

/// bar docs
pub mod bar {
    /// the docs for Bar
    pub struct Bar;
}

//
//  * NO_INLINE
// Used to prevent linking out to separate page or anywhere.
//
// // Example from libcore/prelude
#[doc(no_inline)]
pub use crate::mem::drop;

//  * HIDDEN
// Using this tells rustdoc not to include this in documentation:

// Example from the futures-rs library
#[doc(hidden)]
pub use self::async_await::*;


// Rustdoc - https://doc.rust-lang.org/rustdoc/index.html