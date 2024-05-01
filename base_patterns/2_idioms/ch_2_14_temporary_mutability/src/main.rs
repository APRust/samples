// Often it is necessary to prepare and process some data,
// but after that data are only inspected and never modified.
// The intention can be made explicit by redefining the mutable variable as immutable.
//
// It can be done either by processing data within a nested block or
// by redefining the variable.

fn main() {
    // USING NESTED BLOCK
    let data = {
        let mut data = get_vec();
        data.sort();
        data
    };

    // Here 'data' is immutable



    // USING VARIABLE REBINDING:
    let mut data = get_vec();
    data.sort();
    let data = data;

    // Here `data` is immutable.
}
