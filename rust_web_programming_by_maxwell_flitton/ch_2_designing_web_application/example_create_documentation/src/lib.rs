
use rand::prelude::*;

/// This function generated random f64 number at interval (0, 10)
/// using number generator
///
/// # Arguments
/// * generator (&mut ThreadRng) : the random number generator
/// # Returns
/// * (f64) : random number between (0, 10)
pub fn generate_float(generator: &mut ThreadRng) -> f64 {
    generator.gen::<f64>() * 10.0
}

/// The trait for check that the struct is User
pub trait IsUser {
    /// This function checks that the structs is user
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// * (bool) true if is user, false if not
    fn is_user() -> bool {
        true
    }
}

/// The struct defines a user
///
/// # Attributes
/// * name (String) : name of the user
/// * age (u8) : age of the user
pub struct User {
    name: String,
    age: u8,
}
