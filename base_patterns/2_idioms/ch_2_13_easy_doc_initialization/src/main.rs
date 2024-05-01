// Sometimes there is a struct with multiple or complicated parameters and several methods.
// Each of these methods should have examples.

struct Connection {
    name: String,
    stream: TcpStream,
}

impl Connection {
    /// Sends a request over the connection.
    ///
    /// # Example
    /// ```no_run
    /// # // Boilerplate are required to get an example working.
    /// # let stream = TcpStream::connect("127.0.0.1:34254");
    /// # let connection = Connection { name: "foo".to_owned(), stream };
    /// # let request = Request::new("RequestId", RequestType::Get, "payload");
    /// let response = connection.send_request(request);
    /// assert!(response.is_ok());
    /// ```
    fn send_request(&self, request: Request) -> Result<Status, SendErr> {
        // ...
    }

    /// Oh no, all that boilerplate needs to be repeated here!
    fn check_status(&self) -> Status {
        // ...
    }
}

// Instead of typing all of this boilerplate to create a Connection and Request,
// it is easier to just create a wrapping helper function which takes them as arguments:

struct Connection1{
    name: String,
    stream: TcpStream,
}

impl Connection1 {
    /// Sends a request over the connection.
    ///
    /// # Example
    /// ```
    /// # fn call_send(connection: Connection, request: Request) {
    /// let response = connection.send_request(request);
    /// assert!(response.is_ok());
    /// # }
    /// ```
    fn send_request(&self, request: Request) {
        // ...
    }
}

// Note in the above example the line assert!(response.is_ok());
// will not actually run while testing because it is inside a function which is never invoked.
//
// Advantages
// This is much more concise and avoids repetitive code in examples.
//
// Disadvantages
// As example is in a function, the code will not be tested.
// Though it will still be checked to make sure it compiles when running a cargo test.
// So this pattern is most useful when you need no_run.
// With this, you do not need to add no_run.






