// #[derive(Default)]
pub struct Second {
    value: u64,
    test_str: String
}

//CONSTRUCTOR
impl Second {
    // Constructs a new instance of ['Second'].
    // Note this is an associated function - no self.

    /// Time is seconds.
    ///
    /// # Example
    ///
    /// ```
    /// use ch_2_3_constructor::*;
    ///
    /// let s = Second::new(42, "foo");
    /// assert_eq!(42, s.value());
    /// assert_eq!("foo", s.test_string());
    /// ```
    pub fn new(value: u64, test_str: &str) -> Self {
        Self { value, test_str: test_str.to_string() }
    }
    /// Returns the value in seconds.
    pub fn value(&self) -> u64 {
        self.value
    }
    pub fn test_string (&self) -> &String {
        &self.test_str
    }
}


//DEFAULT_CONSTRUCTOR
impl Default for Second {
    /// Default Time is seconds.
    ///
    /// # Example
    ///
    /// ```
    /// use ch_2_3_constructor::*;
    ///
    /// let default = Second::default();
    /// assert_eq!(0, default.value());
    /// assert_eq!("test", default.test_string());
    ///
    /// ```
    fn default() -> Self {
        Self{ value: 0, test_str: "test".to_string()}
    }
}
