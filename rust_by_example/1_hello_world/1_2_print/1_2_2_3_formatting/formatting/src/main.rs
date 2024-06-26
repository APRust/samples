use std::fmt::{self, Display, Formatter};

struct City {
    name: &'static str,
    //Latitude
    lat: f32,
    //Longitude
    lon: f32,
}

impl Display for City {
    // 'f' is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // 'write!' is like 'format!', but it will write the formatted string into a buffer (first argument)
        write!(
            f,
            "{}: {:.3}'{} {:.3}'{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}


fn main() {
    let foo = 123456789;
    format!("{}", foo);
    println!("{}", foo);

    format!("0x{:X}", foo);
    println!("0x{:X}", foo);

    format!("0o{:o}", foo);
    println!("0o{:o}", foo);

    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ] {
        println!("{}", city);
    }


}
