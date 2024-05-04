// The From and Into traits are inherently linked, and this is actually part of its implementation.
// If you are able to convert type A from type B, then it should be easy to believe
// that we should be able to convert type B to type A.

fn main() {
    // FROM
    // For example we can easily convert a str into a String:
    let my_str = "hello";
    let my_string = String::from(my_str);

    // We can do similar for defining a conversion for our own type:
    use std::convert::From;

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(value: i32) -> Self {
            Number { value }
        }
    }

    let num = Number::from(30);
    println!("Ny number is {:?}", num);


    // INTO
    // The Into trait is simply the reciprocal of the From trait.
    // That is, if you have implemented the From trait for your type,
    // Into will call it when necessary.

    // impl Into<Number> for i32 {
    //     fn into(self) -> Number {
    //         Number { value: self}
    //     }
    // }

    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
