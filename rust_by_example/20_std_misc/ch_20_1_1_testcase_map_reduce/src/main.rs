// Rust makes it very easy to parallelise data processing, without many of the
// headaches traditionally associated with such an attempt.
//
// The standard library provides great threading primitives out of the box.
// These, combined with Rust's concept of Ownership and aliasing rules,
// automatically prevent data races.
//
// The aliasing rules (one writable reference XOR many readable references)
// automatically prevent you from manipulating state that is visible to other threads.
// (Where synchronisation is needed, there are synchronisation primitives like Mutexes
// or Channels.)
//
// In this example, we will calculate the sum of all digits in a block of numbers.
// We will do this by parcelling out chunks of the block into different threads.
// Each thread will sum its tiny block of digits, and subsequently we will sum
// the intermediate sums produced by each thread.
//
// Note that, although we're passing references across thread boundaries,
// Rust understands that we're only passing read-only references, and that thus
// no unsafety or data races can occur. Also because the references we're passing
// have 'static lifetimes, Rust understands that our data won't be destroyed
// while these threads are still running. (When you need to share non-static
// data between threads, you can use a smart pointer like Arc to keep the
// data alive and avoid non-static lifetimes.)

use std::thread;

// const NUMBER_THREADS: u32 = 20;

// fn prepare_data(source: &'static mut String) -> &'static String {
//     println!("source: {source}");
//     let mut new = source.to_string();
//     new.retain(|c| !c.is_whitespace());
//     println!("after one: {source}");
//     let len: u32 = source.len() as u32;
//     let size_of_chunk = len / NUMBER_THREADS;
// 
//     let mut shift = 0u32;
//     for i in 0..19 {
//         new.insert((size_of_chunk * i + shift) as usize, ' ');
//         shift += 1;
//     }
// 
//     println!("after insert ' ' : {}", source);
// 
//     *source = new;
//     source
// }

// This is the `main` thread
fn main() {
    // This is our data to process
    // We will calculate the sum of digits via a threaded map-reduce algorithm
    // Each whitespace separated chunk will be handled in a different thread

    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    // Make a vector to hold the children-threads which we will spawn.
    let mut children = vec![];

    // "MAP" PHASE
    // Divide our data into segments, and apply initial processing

    // split our data into segments for individual calculation
    // each chunk will be a reference (&str) into the actual data
    let chunked_data = data.split_whitespace();

    // Iterate over the data segments.
    // .enumerate() adds the current loop index to whatever is iterated
    // the resulting tuple "(index, element)" is then immediately
    // "destructured" into two variables, "i" and "data_segments" with a
    // "destructuring assignments"
    for (i, data_segments) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segments);

        // Process each data segment in a separate thread
        //
        // spawn() returns a handle to the new thread,
        // which we MUST keep to access the returned value
        //
        // 'move || -> u32' is syntax for a closure that:
        // * takes no arguments ('||')
        // * takes ownership of its captured variables ('move') and
        // * returns an unsigned 32-bit integer ('-> u32')
        //
        // Rust is smart enough to infer the '-> u32' from
        // the closure itself so we could have left that out.
        //

        children.push(thread::spawn(move || -> u32 {
            // Calculate the intermediate sum of this segments:
            let result = data_segments
                // iterate over the characters of our segment
                .chars()
                // convert text-characters to their number value
                .map(|c| c.to_digit(10).expect("should be a digit"))
                // sum the resulting iterate of numbers
                .sum();

            // println! locks stdout, so no text-interleaving occurs
            println!("processed segment {}, result = {}", i, result);
            result
        }));
    }
    //
    //
    // REDUCE PHASE
    //
    //
    // Collect our intermediate results, and combine them into a final result

    // combine each threads intermediate results into a single final sum
    //
    let final_result: u32 = children.into_iter().map(|c| c.join().unwrap()).sum();

    println!("einal sum result: {}", final_result);
}
