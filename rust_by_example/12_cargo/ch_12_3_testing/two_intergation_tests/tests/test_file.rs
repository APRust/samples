use std::fs::OpenOptions;
use std::io::Write;

#[test]
fn test_file() {
    // Opens the file ferris.txt or creates one if it doesn't exist.
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("ferris.txt")
        .expect("Failed to open ferris.txt");

    // Print "Ferris" 5 times.
    for _ in 0..5 {
        file.write_all("Ferris\n".as_bytes())
            .expect("Could not write to ferris.txt");
    }
}
