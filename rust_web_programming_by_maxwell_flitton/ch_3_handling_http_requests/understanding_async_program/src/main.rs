// When it comes to utilizing asynchronous code, there are two main concepts we must understand:
//
// * PROCESSES: A process is a program that is being executed.
// It has its own memory stack, registers for variables, and code.
//
// * THREADS: A thread is a lightweight process that is managed independently by a scheduler.
// However, it does share data, code, and the heap with other threads and the main program.
// However, threads do not share the stack.

use std::thread::JoinHandle;
use std::{thread, time};
use std::any::Any;

fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    2
}

fn process_thread_result (result: Result<i8, Box<dyn Any + Send>>, name: i8) -> i8 {
    match result {
        Ok(result) => {
            println!("the result for {} is {}", name, result);
            result
        }
        Err(result) => {
            if let Some(string) = result.downcast_ref::<String>() {
                println!("the error for {} is: {}", name, string);
            } else {
                println!("there error for {} does not have a message", name);
            }
           0
        }
    }
}

fn main() {
    // Sequential execution
    // let now = time::Instant::now();
    // let one: i8 = do_something(1);
    // let two: i8 = do_something(2);
    // let three: i8 = do_something(3);
    //
    // println!("time elapsed {:?}", now.elapsed());
    // println!("result {}", one + two + three);

    // Concurrent execution
    let now = time::Instant::now();
    let thread_one = thread::spawn(|| do_something(1));
    let thread_two = thread::spawn(|| do_something(2));
    let thread_three = thread::spawn(|| do_something(3));

    let result_one = process_thread_result(thread_one.join(), 1);
    let result_two = process_thread_result(thread_two.join(), 2);
    let result_three = process_thread_result(thread_three.join(), 3);

    println!("time elapsed {:?}", now.elapsed());
    println!("result {}", result_one + result_two + result_three);


}
