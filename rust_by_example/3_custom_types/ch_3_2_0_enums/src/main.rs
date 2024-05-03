enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unload"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \" {}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x = {}, y = {}.", x, y);
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    //If you use a type alias, you can refer to each enum variant via its alias.
    // This might be useful if the enum's name is too long or too generic,
    // and you want to rename it.
    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }
    // Creates a type alias
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    // We can refer to each variant via its alias
    let x = Operations::Add;

    // The most common place you'll see this is in 'impl' blocks using the Self alias
    impl VeryVerboseEnumOfThingsToDoWithNumbers {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Operations::Add => x + y,
                Operations::Subtract => x - y,
            }
        }
    }

    assert_eq!(5, Operations::Add.run(2, 3))
}
