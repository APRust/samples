use ch_2_3_constructor::*;

fn main() {
    let instance = Second::new(33, "test");
    println!("{}", instance.value());

    let default = Second::default();
    println!("default: {}", default.value());
}
