use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(f, "{}: {:.3}°{} {:3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RBG ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}", self.red, self.green, self.blue)
    }
}

pub fn format_print() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    println!("{subject} {verb} {object}", 
        object="the lazy dog.",
        subject="The quick brown fox",
        verb="jumps over"
    );

    // Special formatting can be specified after a `:`
    println!("{} of {:b} people know binary, the other half doesn't.", 1, 2);

    // Right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // Pad numbers with extra zeros.
    println!("{number:>0width$}", number=1, width=6);

    println!("Pi is roughly {pi:.prec$}", pi = 3.141592, prec = 3);

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Olso", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 }
    ].iter() {
        println!("{}", *city);
    }

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0}
    ].iter() {
        println!("{}", *color);
    }
}