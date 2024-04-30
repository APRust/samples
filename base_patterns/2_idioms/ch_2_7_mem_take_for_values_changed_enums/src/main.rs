/// Say we have a &mut MyEnum which has (at least) two variants,
/// A { name: String, x: u8 } and B { name: String }.
/// Now we want  to change MyEnum::A to a B if x is zero, while keeping MyEnum::B intact.
///
/// We can do this without cloning the name.
use std::mem;

fn main() {
    let mut first = MyEnum::A {
        name: "Test".to_string(),
        x: 0,
    };
    a_to_b(&mut first);
    assert_eq!(
        MyEnum::B {
            name: "Test".to_string()
        },
        first
    );

    let mut second = MultiVariateEnum::B {
        name: "New str".to_string(),
    };
    swizzle(&mut second);
    assert_eq!(
        MultiVariateEnum::A {
            name: "New str".to_string()
        },
        second
    )
}

/// NOTE: mem::replace is very similar, but allows us to specify what to replace the value with.
/// An equivalent to our mem::take line would be mem::replace(name, String::new()).
///
/// Note, however, that if we are using an Option and want to replace
/// its value with a None, Option’s take() method provides a shorter and more idiomatic alternative.
///
/// Furthermore, the type you are taking needs to implement the Default trait.
/// However, if the type you’re working with doesn’t implement this, you can instead use mem::replace.
#[derive(Debug, PartialEq)]
enum MyEnum {
    A { name: String, x: u8 },
    B { name: String },
}

fn a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        // This takes out our 'name' and puts in an empty String instead
        // (note that empty strings don't allocate).
        // Then, construct the new enum variant (which will be assigned to '*e'.
        *e = MyEnum::B {
            name: mem::take(name),
        }
    }
}

#[derive(Debug, PartialEq)]
enum MultiVariateEnum {
    A { name: String },
    B { name: String },
    C,
    D,
}

fn swizzle(e: &mut MultiVariateEnum) {
    use MultiVariateEnum::*;
    *e = match e {
        // Ownership rules do not allow taking `name` by value, but we cannot
        // take the value out of a mutable reference, unless we replace it:
        A { name } => B {
            name: mem::take(name),
        },
        B { name } => A {
            name: mem::take(name),
        },
        C => D,
        D => C,
    }
}
