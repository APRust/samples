// The async and await syntax manages the same concepts covered in the previous section; however,
// there are some nuances. Instead of simply spawning off threads,
// we create FUTURES and then manipulate them as and when needed.

use async_std;
use futures::executor::block_on;
use futures::future::join_all;
use futures::join;
use std::time::Duration;
use std::vec::Vec;
use std::{thread, time};

async fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    2
}

fn main() {
    // FIRST example
    // let now = time::Instant::now();
    // let future_one = do_something(1);
    // let outcome = block_on(future_one);
    // println!("time elapsed: {:?}", now.elapsed());
    // println!("Here is the outcome: {}", outcome);

    // SECOND example with extra sleep func
    // let now = time::Instant::now();
    // let future_one = do_something(1);
    // let two_seconds = time::Duration::new(2, 0);
    // thread::sleep(two_seconds);
    // let outcome = block_on(future_one);
    // println!("time elapsed: {:?}", now.elapsed());
    // println!("Here is the outcome: {}", outcome);

    // THIRD example with using the 'await' syntax
    // let now = time::Instant::now();
    // let future_three = async {
    //     let outcome_one = do_something(2).await;
    //     let outcome_two = do_something(3).await;
    //     return outcome_one + outcome_two;
    // };
    //
    // let future_outcome = block_on(future_three);
    // println!("time elapsed: {:?}", now.elapsed());
    // println!("Here is the outcome: {:?}", future_outcome);

    // FOUR example with using 'join'
    // let future_four = async {
    //     let outcome_one = do_something(2);
    //     let outcome_two = do_something(3);
    //     let results = join!(outcome_one, outcome_two);
    //     return results.0 + results.1;
    // };
    //
    // let now = time::Instant::now();
    // let result = block_on(future_four);
    // println!("time elapsed: {:?}", now.elapsed());
    // println!("here is the result: {:?}", result);

    // In the preceding code, we can see that the join macro returns a tuple
    // of the results and that we
    // unpack the tuple to give us the same result.
    // However, if we do run the code, we can see that although
    // we get the result that we want, our future execution does not speed up and
    // is still stuck at just above
    // 4 seconds. This is because a future is not being run using an async task.
    // We will have to use async
    // tasks to speed up the execution of our futures.
    //
    // We can achieve this by carrying out the following steps:

    // 1. Create the futures needed.
    // 2. Put them into a vector.
    // 3. Loop through the vector, spinning off tasks for each future in the vector.
    // 4. Join the async tasks and sum the vector.

    // To join all our futures at the same time, we will have to use another crate
    // to create our own asynchronous join function by using the async_std crate.
    // We define this crate in the Cargo.toml file with the following code:
    // async-std = "1.11.0"

    // Now that we have the async_std crate, we can import what we need to
    // carry out the approach laid out in Figure 3.2, by importing what
    // we need at the top of the main.rs file with the following code:
    // use std::vec::Vec;
    // use async_std;
    // use futures::future::join_all;

    // FIVE example
    let async_outcome = async {
        // 1.
        let mut futures_vec = Vec::new();
        let future_four = do_something(4);
        let future_five = do_something(5);
        // 2.
        futures_vec.push(future_four);
        futures_vec.push(future_five);

        // 3.
        let handles = futures_vec
            .into_iter()
            .map(async_std::task::spawn)
            .collect::<Vec<_>>();

        // 4.
        let results = join_all(handles).await;
        return results.into_iter().sum::<i8>();
    };

    let now = time::Instant::now();
    let result = block_on(async_outcome);
    println!("time elapsed: {:?}", now.elapsed());
    println!("here is the result: {:?}", result);
    let a = 4;
    let result = (1..=a).fold(0, |acc, x| acc + 321);
    println!("{}", result);
}
