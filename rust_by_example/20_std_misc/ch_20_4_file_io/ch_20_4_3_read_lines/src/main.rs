// A NAIVE_APPROACH
// This might be a reasonable first attempt for a beginner's first implementation
// for reading lines from a file.

use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

// Since the method lines() returns an iterator over the lines in the file,
// we can also perform a map inline and collect the results, yielding a more
// concise and fluent expression.

fn read_lines_fluent(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file_reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect()
}

// Note that in both examples above, we must convert the &str reference returned
// from lines() to the owned type String, using .to_string() and String::from respectively.

// A MORE EFFICIENT APPROACH
//
// Here we pass ownership of the open File to a BufReader struct.
// BufReader uses an internal buffer to reduce intermediate allocations.
//
// We also update read_lines to return an iterator instead of allocating
// new String objects in memory for each line.

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped is a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines_efficient<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines_efficient("./hosts.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten(){
            println!("{}", line);
        }
    }
}

// $ echo -e "127.0.0.1\n192.168.0.1\n" > hosts.txt
// $ rustc main.rs && ./main

// (Note that since File::open expects a generic AsRef<Path> as argument,
// we define our generic read_lines() method with the same generic constraint,
// using the where keyword.)
//
// This process is more efficient than creating a String in memory
// with all of the file's contents. This can especially cause performance issues
// when working with larger files.

























