// Construct an object with calls builder helper.

#[derive(Debug, PartialEq)]
pub struct Foo {
    // Lost of complicated fields
    bar: String,
}

impl Foo {
    //noinspection ALL
    // Этот метод поможет пользователям найти конструктор
    pub fn builder() -> FooBuilder {
        FooBuilder::default()
    }
}

#[derive(Default)]
pub struct FooBuilder {
    // Probably lots of optional fields
    bar: String,
}

impl FooBuilder {
    pub fn new() -> FooBuilder {
        // Set the minimal required fields of Foo.
        FooBuilder {
            bar: String::from("X"),
        }
    }

    pub fn name(mut self, bar: String) -> FooBuilder {
        // Set the name no the builder itself, and return the builder by value
        self.bar = bar;
        self
    }

    // If we can get away with not consuming the builder here, that is an
    // advantage. It means we can use the FooBuilder as a template for constructing many Foos
    pub fn build(self) -> Foo {
        // Create a Foo from the FooBuilder, applying all settings in FooBuilder to Foo
        Foo { bar: self.bar }
    }
}

fn main() {
    let foo = Foo {
        bar: String::from("Y"),
    };

    let foo_from_builder = Foo::builder()
        .name(String::from("Y"))
        .name(String::from("Y"))
        .build();

    assert_eq!(foo, foo_from_builder);
}
