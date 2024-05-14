// The 'newtype' idiom gives compile time guarantees that the right type of value
// is supplied to a program.

// For example, an age verification function that checks age in years, must be given
// a value of type 'Years'

#![allow(dead_code)]
struct Years(u16);
struct Days(u16);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn is_adult(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(25);
    let age_days = age.to_days();
    println!("Is an adult? {}", is_adult(&age));
    println!("Is an adult? {}", is_adult(&age_days.to_years()));

    // Error! mismatch types.
    // println!("Is an adult? {}", is_adult(&age_days));

    // To obtain the newtype's value as the base type, you may use the tuple or
    // destruction syntax like so:

    let years = Years(42);
    let years_as_primitive = years.0; // Tuple
    println!("{years_as_primitive}");

    let Years(value) = years; // Destructing
    println!("{value}");
}
