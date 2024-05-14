// Multiple bounds for a single type can be applied with a ' + '. Like normal,
// different types are separated with ' , '.

use std::fmt::{Debug, Display, Formatter};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: '{:?}'", t);
    println!("Display: '{}'", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: '{:?}'", t);
    println!("u: '{:?}'", u);
}


//noinspection ALL
fn main() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    // compare_prints(&array);
    // compare_prints(&vec);

    compare_types(&array, &vec);


   // Implement Display for Vec<i32> though 'new type' pattern
    #[derive(Debug)]
    struct Wrapper(Vec<i32>);

    impl Display for Wrapper {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }

    }
    let wrap_vec = Wrapper (vec![1, 2, 3]);
    compare_prints(&wrap_vec);
}
