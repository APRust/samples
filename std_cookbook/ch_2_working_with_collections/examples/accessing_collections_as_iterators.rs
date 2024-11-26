use std::ops::Index;

#[allow(unused_doc_comments)]
fn main() {
    let names = vec!["Joe", "Miranda", "Alice"];
    // Iterators can be accessed in many ways.
    // Nearly all collections implement .iter() for this purpose
    let mut iter = names.iter();
    // A strings itself is not iterable, but its characters are
    let mut alphabet = "ABCDEFGHIJKLMNOPQRSRUVWXYZ".chars();
    // Ranges are also (limited) iterators
    let nums = 0..10;
    // You can even create infinite iterators!
    let all_nums = 0..;

    // As the name says, you can iterate over iterators
    // This will consume the iterator
    for num in nums {
        print!("{num}");
    }
    // nums is no longer usable
    println!();

    // Get the index of the current item.
    for (index, letter) in "abc".chars().enumerate() {
        println!("#{}. letter in the alphabet is {}", index + 1, letter);
    }

    /// Access individual items
    // going through an iterator step by step
    if let Some(name) = iter.next() {
        println!("First name is {}", name);
    }
    if let Some(name) = iter.next() {
        println!("Second name is {}", name);
    }
    if let Some(name) = iter.next() {
        println!("Third name is {}", name);
    }
    if iter.next().is_none() {
        println!("No names left");
    }

    // Arbitrary access to an item in the iterator
    let letter = alphabet.nth(3);
    if let Some(letter) = letter {
        println!("the fourth letter in the alphabet is {}", letter);
    }
    // This works by consuming all items up to a point
    let current_first = alphabet.nth(0);
    if let Some(current_first) = current_first {
        // This will NOT print 'A'
        println!(
            "The first item in the iterator is currently: {}",
            current_first
        );
    }
    let current_first = alphabet.nth(0);
    if let Some(current_first) = current_first {
        // This will print new current
        println!(
            "The first item in the iterator is currently: {}",
            current_first
        );
    }

    // Accessing the last item; This will consume the entire iterator
    let last_letter = alphabet.last();
    if let Some(last_letter) = last_letter {
        println!("The last letter of the alphabet is {}", last_letter);
    }

    /// Collect the iterator into a collection:
    // This require annotation of which collection we want
    // The following two are equivalent:
    let nums: Vec<_> = (1..10).collect();
    println!("nums: {:?}", nums);
    let nums = (1..10).collect::<Vec<_>>();
    println!("nums: {:?}", nums);

    /// Change which items are being iterated over:
    // Taking only the first n items
    // This is often used to make an infinite iterator finite
    let nums: Vec<_> = all_nums.take(5).collect();
    println!("The first five numbers are: {:?}", nums);

    // Skip the first few items
    let nums: Vec<_> = (0..11).skip(2).collect();
    println!("The last 9 numbers in a range from zero to 10: {:?}", nums);

    // take and skip accept predicate in the form of take_while and skip_while
    let nums: Vec<_> = (0..).take_while(|x| x * x < 50).collect();
    println!(
        "All positive numbers that are less than 50 when squared: {:?}",
        nums
    );

    // This is useful to filter an already sorted vector
    let names = ["Alfred", "Andy", "Jose", "Luke"];
    let names: Vec<_> = names.iter().skip_while(|x| x.starts_with('A')).collect();
    println!("Names which don't start with 'A': {:?}", names);

    // Filtering iterators
    let countries = ["U.S.A.", "Germany", "Italy", "India", "Pakistan", "Burma"];
    let countries_with_i: Vec<_> = countries
        .iter()
        .filter(|country| country.contains(&['i', 'I']))
        .collect();
    println!("countries_with_i: {:?}", countries_with_i);

    /// Check if an iterator contains an element
    // Find the first element that satisfied a condition
    if let Some(country) = countries.iter().find(|country| country.starts_with('I')) {
        println!("The first country start with 'I': {:?}", country);
    }

    // Don't get the searched item but rather it's index
    if let Some(country) = countries
        .iter()
        .position(|country| country.starts_with('I'))
    {
        println!("The index first country start with 'I': {:?}", country);
    }
    
    // Check if at least one item satisfies a condition
    let are_any = countries.iter().any(|c| c.len() == 5);
    println!("Is the at least country with five letters? {:?}", are_any);
    
    // Check if ALL items satisfy a condition
    let are_all= countries.iter().all(|c| c.len() == 5);
    println!("Is the ALL countries with five letters? {:?}", are_all);
    
    /// Useful operations for numeric items:
    let sum: i32 = (1..11).sum();
    let product: i32 = (1..11).product();
    println!(
        "When operating on the first ten positive numbers\n\
        their sum is {} and \n\
        their product is {}.",
        sum, product
    );
    
    let max = (1..11).max();
    let min = (1..11).min();
    if let Some(max) = max {
        println!("They have a highest number and it is: {}", max)
    }
    if let Some(min) = min {
        println!("They have a smallest number and it is: {}", min)
    }
    
    /// Combine iterators:
    
    
    /// Apply functions to all items:
    
    
    /// The real strength of iterators comes from combining them:
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    

    println!()
}
