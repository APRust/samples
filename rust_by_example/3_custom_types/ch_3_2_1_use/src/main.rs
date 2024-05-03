// An attribute to hide for unused code.
#![allow(dead_code)]

use crate::Work::Civilian;

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    //Explicitly 'use' each name so they are available without manual scoping
    use crate::Status::{Poor, Rich};
    // Automatically 'use' each name inside 'Work'
    use crate::Work::*;

    // Equivalent to 'Status::Poor'.
    let status = Poor;

    // Equivalent to 'Work::Civilian'.
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit 'use' above
        Rich => {
            println!("The rich have lots of money!")
        }
        Poor => {
            println!("The poor have no money...")
        }
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilian work!"),
        Soldier => println!("Soldier fight!"),
    }
}
