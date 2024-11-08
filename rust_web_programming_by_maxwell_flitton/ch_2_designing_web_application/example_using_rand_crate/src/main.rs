use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    // Generate f64 (0.0, 10.0)
    let f64_random = rng.gen::<f64>() * 10.0;
    println!("{f64_random}");

    // Generate i32 [-10, 9]
    let i32_random = rng.gen_range(-10..=9);
    println!("{i32_random}");

    // Generate u32 (0, 10)
    let u32_random: u32 = rng.gen_range(0..10);
    println!("{u32_random}");
}
