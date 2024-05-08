#[derive(Debug)]
enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The 'if let' construct reads: "if 'let' destructures 'number' into 'Some(i),
    // evaluate the block ('{}').
    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }

    // Provide an altered failing condition.
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
        // Destructure failed. Evaluated an 'else if' condition to see if the
        // alternate failure branch should be taken:
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    // In same way, 'if let' can be used to match any enum value:

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // Variable a matches Foo:Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also works with 'if let'
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred! value : {}", value)
    }


    // Another benefit is that if let allows us to match non-parameterized enum variants.
    // This is true even in cases where the enum doesn't implement or derive PartialEq.
    // In such cases if Foo::Bar == a would fail to compile,
    // because instances of the enum cannot be equated, however if let will continue to work.

    // This enum purposely neither implement nor derives PartialEq.
    // That is why comparing Foo::Bar == a fails below.
    enum Foo1 {
        Bar,
    }

    let a = Foo1::Bar;
    // Variable a matches Foo::Bar - FAIL
    // if Foo::Bar == a { println!("a is foobar")};

    if let Foo1::Bar = a {
        println!("a is foobar")
    };
}
