use std::fmt;

type Lat = f64;
type Long = f64;

enum Location {
    Anonymous,
    Known(Lat, Long),
    Unknown,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Anonymous => write!(f, "anonymous"),
            Self::Unknown => write!(f, "unknown"),
            Self::Known(lat, long) => write!(f, "{} lat / {} long", lat, long),
        }
    }
}

fn main() {
    let address = Location::Unknown;
    println!("Location: {}", address);

    let address = Location::Anonymous;
    println!("Location: {}", address);

    let address = Location::Known(28.608295, -80.604177);
    println!("Location: {}", address);
}
