// MOTIVATION
// The primary motivation for newtypes is abstraction.
// It allows you to share implementation details between types while
// precisely controlling the interface. By using a newtype rather than exposing
// the implementation type as part of an API, it allows you to change implementation
// backwards compatibly.
//
// Newtypes can be used for distinguishing units, e.g., wrapping f64 to give
// distinguishable Miles and Kilometres.
//
// ADVANTAGES
// The wrapped and wrapper types are not type compatible (as opposed to using type),
// so users of the newtype will never ‘confuse’ the wrapped and wrapper types.
//
// Newtypes are a zero-cost abstraction - there is no runtime overhead.
//
// The privacy system ensures that users cannot access the wrapped type
// (if the field is private, which it is by default).

use std::fmt::{Display, Formatter};

// Create Newtype Password to override the Display trait for String
struct Password(String);

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "**********")
    }
}

fn main() {
    let unsecured_password: String = "ThisIsMyPassword".to_string();
    let secured_password: Password = Password(unsecured_password.clone());
    println!("unsecured_password: {unsecured_password}");
    println!("secured_password: {secured_password}");
}
