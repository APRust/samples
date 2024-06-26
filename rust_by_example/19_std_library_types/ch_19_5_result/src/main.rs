// We've seen that the Option enum can be used as a return value from functions
// that may fail, where None can be returned to indicate failure.
// However, sometimes it is important to express why an operation failed.
// To do this we have the Result enum.
//
// The Result<T, E> enum has two variants:
//
//  * Ok(value) which indicates that the operation succeeded,
// and wraps the value returned by the operation. (value has type T)
//
//  * Err(why), which indicates that the operation failed, and wraps why,
// which (hopefully) explains the cause of the failure. (why has type E)
//

use crate::checked::{MathError, MathResult};
use std::ptr::read;

mod checked {
    // Mathematical "errors" we want to catch

    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            // This operation would `fail`, instead let's return the reason of
            // the failure wrapped in `Err`
            Err(MathError::DivisionByZero)
        } else {
            // This operation is valid, return the result wrapped in `Ok`
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }
}

// `op(x,y)` === `sqrt(ln(x / y))`
//
// BASE REALIZATION
fn op(x: f64, y: f64) -> f64 {
    // This is a three level match pyramid!
    match checked::div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}

// REALIZATION WITH ` ? `

// Intermediate function
fn op_(x: f64, y: f64) -> MathResult {
    // if `div` "fails", then `DivisionByZero` will be return
    let ratio = checked::div(x, y)?;

    // If `ln` "fails", then `NonPositiveLogarithm` will be `return
    let ln = checked::ln(ratio)?;

    checked::sqrt(ln)
}

pub fn op_with_question(x: f64, y: f64) {
    match op_(x, y) {
        Err(why) => panic!(
            "{}",
            match why {
                MathError::NonPositiveLogarithm => "logarithm of non-positive number",
                MathError::DivisionByZero => "division by zero",
                MathError::NegativeSquareRoot => "square root of negative number",
            }
        ),
        Ok(value) => println!("{}", value),
    }
}

fn main() {
    // Will this fail?
    // println!("{}", op(1.0, 10.0));
    op_with_question(1.0, 10.0);
}
