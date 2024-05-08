// Branching with if-else is similar to other languages.
// Unlike many of them, the boolean condition doesn't need to be surrounded by parentheses,
// and each condition is followed by a block. if-else conditionals are expressions,
// and, all branches must return the same type.

fn main() {
    let n = 11;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n)
    } else {
        println!("{} is zero", n)
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        // This expression returns an 'i32'.
        10 * n
    } else {
        println!(", and is big number, halve the number");

        //This expression must return an 'i32' as well.
        n / 2
    };

    println!("{} -> {}", n, big_n);
}
