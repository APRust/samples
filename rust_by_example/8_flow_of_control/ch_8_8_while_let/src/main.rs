fn main() {
    // WHILE LET
    println!("WHILE LET decision... ");
    let mut optional = Some(0);

    // This reads: "while 'let' destructures 'optional' into
    // 'Some(i)', evaluate the block ('{}'). Else 'break'.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greatest than 9, quit!");
            optional = None;
        } else {
            println!("'i' is '{:?}'. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ doesn't require explicitly handling the failing case
    }
    // ^ 'if let' had additional optional 'else'/'else if' clauses.
    // 'while let' does not have these

    // CLASSIC
    println!("CLASSIC decision... ");
    let mut optional = Some(9);
    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greatest than 9, quit!");
                    optional = None;
                } else {
                    println!("'i' is '{:?}'. Try again.", i);
                    optional = Some(i + 1);
                }
            }
            // Quit the loop when the destructure fails:
            _ => {
                break;
            }
        }
    }
}
