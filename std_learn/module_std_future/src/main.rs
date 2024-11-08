#![feature(future_join)]

use core::future::poll_fn;
use std::future;
use std::future::join;
use std::task::{Context, Poll};

/// KEYWORD
/// - async
// Use 'async' in front of fn, closure, or a block to turn the marked code into a Future.
// As such the code will be run immediately, but will only be evaluated when the
// returned future is '.await'ed.

/// - await
// Suspend execution until the result of a Future is ready.
// .awaiting a future will suspend the current function's execution until the executor has
// run the future to completion.

#[tokio::main]
async fn main() {
    /// MACROS
    /// pub macro join($($fut:expr), + $(,)?) { ... } - Experimental
    // - Polls multiple futures simultaneously, returning a tuple of all results once complete.
    // While join!(a, b).await is similar to (a.await, b.await), join! polls both futures
    // concurrently and is therefore more efficient.

    // join! is variadic, so you can pass any number of futures:

    async fn one() -> usize {
        1
    }
    async fn two() -> usize {
        2
    }
    async fn three() -> usize {
        3
    }

    let x = join!(one(), two(), three()).await;
    assert_eq!(x, (1, 2, 3));

    /// STRUCTS
    /// - Pending
    // Creates a future which never resolves, representing a computation that never finishes.

    /// - PollFn
    // A Future that wraps a function returning Poll.

    /// - Ready
    // A Future that is immediately ready with a value.

    /// - AsyncDropInPlace - Experimental
    // A Future returned by the async_drop_in_place

    /// TRAITS
    /// Future
    // A future represents an asynchronous computation obtained by use of async.

    /// IntoFuture
    // Conversion into a Future

    /// AsyncDrop - Experimental
    // Custom code within the asynchronous destructor

    /// FUNCTIONS
    /// pub fn pendind<T>() -> Pending<T>

    // Creates a future which never resolves, representing a computation that never finishes

    // let future = future::pending();
    // let () = future.await;
    // unreachable!();

    // ----------------------------------------

    /// pub fn poll_fn<T, F>(f: F) -> PollFn<F>
    /// where
    /// F: FnMut(&mut Context<'_>) -> Poll<T>

    // Creates a futures that wraps a function returning Poll.

    fn read_line(_cx: &mut Context<'_>) -> Poll<String> {
        Poll::Ready("Hello, World!".into())
    }
    let read_future = poll_fn(read_line);
    assert_eq!(read_future.await, "Hello, World!".to_owned());

    /// -----------------------------------------

    /// pub fn ready<T>(t: T) -> Ready<T>
    // Creates a future that is immediately ready with a value.
    // Futures created through this function are functionally similar to those created
    // through async {}. The main difference is that futures crated through this function
    // are named and implement Unpin.
    let a = future::ready(1);
    assert_eq!(a.await, 1);

    ///   -----------------------------------------

    /// pub fn async_drop<T>(value: T) -> AsyncDropOwning<T>
    /// Experimental
    // Asynchronously drops a value by running AsyncDrop::async_drop on a value and
    // ist fields recursively
    let ();
}
