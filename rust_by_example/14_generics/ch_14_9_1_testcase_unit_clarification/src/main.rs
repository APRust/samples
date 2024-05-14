use std::marker::PhantomData;
use std::ops::Add;

// Create void enumeration to define unit types
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

// 'Length' is a type with phantom type parameter 'Unit', and is not generic over
// the length type (that is 'f64')
//
// 'f64' already implements the 'Clone' and 'Copy' traits.
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

// The 'Add' trait defines the behavior of the '+' operator.
impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    // add() returns a new 'Length' struct containing the sum
    fn add(self, rhs: Self) -> Self::Output {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    // Specifies 'one_foot' to have phantom type parameter 'Inch'
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    // 'one_meter' has phantom type parameter 'Mm'.
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    // '+' calls the 'add()' method we implemented for 'Length<Unit>'.
    //
    // Since 'Length' implements 'Copy', 'add()' does not consume
    // 'one_foot' and 'one_meter' but copies them into 'self' and 'rhs'.

    let two_foots = one_foot + one_foot;
    let two_meters = one_meter + one_meter;
    
    println!("one_foot + one_foot: {:?}", two_foots.0);
    println!("one_meters + one_meters: {:?}", two_meters.0);

    //Error! mismatch types
    // let one_feter = one_meter + one_foot;
    // println!("inch_var1 + mm_var1: {:?}", inch_var1 + mm_var1);
}
