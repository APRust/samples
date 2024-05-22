fn main() {
    // An Iter::map operation might fail, for example:
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("Results: {:?}", numbers);

    // Let's step through strategies for handling this:

    ///  IGNORE THE FAILED ITEMS WITH ` FILTER_MAP() `
    // filter_map() calls a function and filters out the results that ara None.
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results: {:?}", numbers);

    /// COLLECT THE FAILED ITEMS WITH MAP_ERR() AND FILTER_MAP()
    // map_err calls a function with the error, so by adding that to the previous filter_map
    // solution we can save them off to the side while iterating.
    let strings = vec!["42", "tofu", "93", "18"];
    let mut errors = vec![];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<u8>())
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);

    /// FAIL THE ENTIRE OPERATION WITH COLLECT()
    // `Result` implements `FromIterator` so that a vector of results (Vec<Result<T, E>>)
    // can be turned into a result a vector (Result<Vec<T>, E>) . Once an Result::Err is found,
    // the iteration will terminate.
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("Results: {:?}", numbers);

    /// COLLECT ALL VALID VALUES AND FAILURES WITH PARTITION()
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);

    // When you look at the results, you'll note that everything is still wrapped in `Result`
    // A little more boilerplate is needed for this

    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
