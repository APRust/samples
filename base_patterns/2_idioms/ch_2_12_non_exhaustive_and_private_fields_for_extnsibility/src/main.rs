// A small set of scenarios exist where a library author may want
// to add public fields to a public struct or new variants to an enum
// without breaking backwards compatibility.
//
// Rust offers two solutions to this problem:
//
// Use #[non_exhaustive] on structs, enums, and enum variants.
// For extensive documentation on all the places where #[non_exhaustive] can be used,
// see the docs.
//
// You may add a private field to a struct to prevent it from being directly instantiated
// or matched against (see Alternative)

mod a {
    // Public struct
    #[non_exhaustive]
    pub struct S {
        pub foo: i32,
    }

    #[non_exhaustive]
    pub enum AdmitMoreVariants {
        VariantA,
        VariantB,
        #[non_exhaustive]
        VeriantC {
            a: String,
        },
    }
}

fn print_matched_variants(s: a::S) {
    // Because S is '#[non_exhaustive]', it cannot be named here and
    // we must use '..' in the pattern.
    let a::S { foo: _, .. } = s;

    let some_enum = a::AdmitMoreVariants::VariantA;
    match some_enum {
        a::AdmitMoreVariants::VariantA => println!("It's an A"),
        a:: AdmitMoreVariants::VariantB => println!("It's an B"),

        // .. required because this variant is non-exhaustive as well
        a::AdmitMoreVariants::VeriantC {a, ..} => println!("It's a C"),

        // The wildcard match is required because more variants may be added in the future
        _ => println!("It's a new variant")
    }
}

// Alternative: Private fields for structs
// #[non_exhaustive] only works across crate boundaries.
// Within a crate, the private field method may be used.
//
// Adding a field to a struct is a mostly backwards compatible change.
// However, if a client uses a pattern to deconstruct a struct instance,
// they might name all the fields in the struct and adding a new one would break that pattern.
// The client could name some fields and use .. in the pattern,
// in which case adding another field is backwards compatible.
// Making at least one of the struct’s fields private
// forces clients to use the latter form of patterns, ensuring that the
// struct is future-proof.

// pub struct S {
//     pub a: i32,
//     // Because `b` is private, you cannot match on `S` without using `..` and `S`
//     //  cannot be directly instantiated or matched against
//     _b: (),
// }

fn main() {
    print_matched_variants(
        a::S { foo: 5 }
    )
}
