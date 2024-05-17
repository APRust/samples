// Annotation of lifetimes in trait methods basically are similarly to functions.
// Note that `impl` may have annotation of lifetime too.

#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

// Annotate lifetimes to impl.
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}

fn main() {
    let b = <Borrowed as Default>::default();
    println!("b is {:?}", b);
}
