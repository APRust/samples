// If a fallible function consumes (moves) an argument,
// return that argument back inside an error.

pub fn send(value: String) -> Result<(), SendError> {
    println!("using {value} in a meaningful way");
    // Simulate non-deterministic fallible action.
    use std::time::SystemTime;
    let period = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    if period.subsec_nanos() % 2 == 1 {
        Ok(())
    } else {
        Err(SendError(value))
    }
}

pub struct SendError(String);

fn main() {
    let mut value = "imagine this is very long string".to_string();
    let mut returned_value = "".to_string();

    let success = 's: {
        // Try to send value two times.
        for _ in 0..2 {
            value = match send(value) {
                Ok(()) => break 's true,
                Err(SendError(value)) => value,
            }
        }
        // In case of error you may want to try some alternative way or to retry action
        // in case of non-deterministic function. But if the argument is always consumed,
        // you are forced to clone it on every call, which is not very efficient.
        println!("value returned from SendError after fail send(): {value}");

        // for up source value back
        // returned_value = value;
        false
    };

    // println!("UP: {returned_value}");
    println!("success: {success}");
}
