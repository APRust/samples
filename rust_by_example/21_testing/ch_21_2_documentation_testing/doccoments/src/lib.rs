// The primary way of documenting a Rust project is through annotating the source code.
// Documentation comments are written in CommonMark Markdown specification and support
// code blocks in them. Rust takes care about correctness, so these code blocks
// are compiled and used as documentation tests.

/// First line is a short summary describing function
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `doccomments` crate
///
/// ```
/// let result = doccoments::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Usually doc comments may include sections "Examples", "Panics" and "Failures".
///
/// The next function divides two numbers.
///
/// # Examples
///
/// ```
/// let result = doccoments::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust, should_panic
/// // panics on division by zero
/// doccoments::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}

// Главная цель документационных тестов - служить примерами предоставляемой функциональности,
// что является одной из самых важных рекомендаций.
// Это позволяет использовать примеры из документации в качестве полных фрагментов кода.
// Но использование ? приведёт к ошибке компиляции,
// так как функция main возвращает () (unit). На помощь приходит возможность скрыть
// из документации некоторые строки исходного кода:
// можно написать fn try_main() -> Result<(), ErrorType>, скрыть её и вызвать её в
// скрытом main с unwrap. Звучит сложно? Вот пример:

/// Использование скрытой `try_main` в документационных тестах.
///
/// ```
/// # // скрытые строки начинаются с символа `#`, но они всё ещё компилируемы!
/// # // эта линия оборачивает тело функции, которое отображается в документации
/// # fn try_main() -> Result<(), String> {
/// let res = doccoments::try_div(10, 2)?;
/// # Ok(()) // возвращается из try_main
/// # }
/// # fn main() { // начало `main` которая выполняет `unwrap()`
/// #    try_main().unwrap(); // вызов `try_main` и извлечение результата
/// #                         // так что в случае ошибки этот тест запаникует
/// # }
/// ```
///
///```
///let c = doccoments::try_div(10, 5).unwrap();
/// assert_eq!(c, 2);
/// ```
///
/// ```
/// let result = doccoments::try_div(10, 0);
/// assert_eq!(result, Err(String::from("Деление на 0")));
/// ```
///
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Деление на 0"))
    } else {
        Ok(a / b)
    }
}

























