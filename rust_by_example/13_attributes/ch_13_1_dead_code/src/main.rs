// The compiler provides a 'dead_code' lint that will warn about unused functions.
// Ann attribute can be used to disable the lint.

fn used_function() {}

// '#[allow(dead_code)]' is an attribute that disabled the 'dead_code' lint
#[allow(dead_code)]
fn unused_function() {}

#[allow(dead_code)]
fn noisy_unused_function() {}

fn main() {
    used_function()
}
