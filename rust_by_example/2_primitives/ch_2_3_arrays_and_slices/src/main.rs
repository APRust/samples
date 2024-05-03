use std::mem;

// This function borrows a slice.
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // Fixed-sizes array
    let xs = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [0; 500];

    // Indexing stars at 0.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // 'len' returns the count of elements in the array.
    println!("Numbers of elements in array: {}", xs.len());

    // Arrays are stack allocated
    println!("Arrays occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);

    // Example of empty slice '&[]':
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => {
                println!("{}: {}", i, xval)
            }
            None => {
                println!("Slow down! {} is too far!", i)
            }
        }
    }

    // Compile time index out of bound
    // println!("{}", xs[5]);

    // Runtime index out of bound
    // println!("{}", xs[..][5])
}
