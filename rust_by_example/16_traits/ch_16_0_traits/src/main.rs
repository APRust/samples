// A trait is a collection of methods defined for an unknown type: Self.
// They can access other methods declared in the same trait.
//
// Traits can be implemented for any data type. In the example below, we define Animal,
// a group of methods. The Animal trait is then implemented for the Sheep data type,
// allowing the use of methods from Animal with a Sheep.

struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    // Associated function signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definition.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor method can use the implementor's trait methods.
            println!("{} is already haked...", self.name());
        } else {
            println!("{} gets a haricut!", self.name);
            self.naked = true;
        }
    }
}

// Implement the `Animal` trait for `Sheep`
impl Animal for Sheep {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: &'static str) -> Self {
        Sheep { name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaaah?"
        } else {
            "baaaaah!"
        }
    }

    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    // Type annotation is necessary in this case.
    let mut dolly : Sheep = Animal::new("Dolly");
    // or...
    // let mut dolly = <Sheep as Animal>::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}
