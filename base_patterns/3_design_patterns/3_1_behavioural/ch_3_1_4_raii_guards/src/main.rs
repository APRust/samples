// RAII stands for “Resource Acquisition is Initialisation” which is a terrible name.
// The essence of the pattern is that resource initialisation is done in the constructor
// of an object and finalisation in the destructor. This pattern is extended in Rust
// by using a RAII object as a guard of some resource and relying on the
// type system to ensure that access is always mediated by the guard object.

// Mutex guards are the classic example of this pattern from the std library (this is a simplified
// version of the real implementation):

use std::ops::Deref;

struct Foo {}

impl Foo {
    fn foo() {}
}

struct Mutex<T> {
    // We keep a reference to our data: T here.
    //...
}

struct MutexGuard<'a, T: 'a> {
    data: &'a T,
    //..
}

// Locking the mutex is explicit
impl<T> Mutex<T> {
    fn lock(&self) -> MutexGuard<T> {
        // Lock the underlying OS mutex.
        // ..

        // MutexGuard keeps a reference to self
        MutexGuard {
            data: self,
            // ..
        }
    }
}

// Destructor for unlocking the mutex
impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        // Unlock the underlying OS mutex
        //..
    }
}

// Implementing Deref means we can treat MutexGuard like a pointer to T.
impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

fn baz(x: Mutex<Foo>) {
    let xx = x.lock();
    xx.foo(); // foo is a method on Foo.
    // The borrow checker ensures we can't store a reference to the underlying
    // Foo which will outlive the guard xx
    // x is unlocked when we exit this function and xx's destructor is executed
}

fn main() {
    println!("Hello, world!");
}
