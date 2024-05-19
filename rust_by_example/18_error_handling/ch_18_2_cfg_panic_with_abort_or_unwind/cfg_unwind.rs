// Here is another example focusing on rewriting drink() and explicitly use the `unwind` keyword

#[cfg(panic = "unwind")]
fn ah() {
    println!("Spit is out!!!!");
}

#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("This is not your party. Run!!!!!");
}

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        ah()
    } else {
        println!("Some refreshing {} is all I need. ", beverage);
    }
}

fn main() {
    drink("water");
    drink("lemonade");
}

// rustc cfg_unwind.rs -C panic=unwind && ./cfg_unwind