use crate::List::*;
enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty List
    fn new() -> List {
        // 'Nil' has type 'List'
        Nil
    }

    // Consume a list, and the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // 'Cons' also has type List
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        match self {
            Cons(_,  tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match self {
            Cons(head,  tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    //Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify())
}
