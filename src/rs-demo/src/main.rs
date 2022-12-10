trait Description {
    fn describe(&self) -> String {
        String::from("an object flying through space")
    }
}

struct Satellite {
    name: String,
    velocity: f64, // miles per second
}

impl Description for Satellite {}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32, // miles
}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!(
            "the {} flying at {} miles height with {} crew members on board!",
            self.name, self.altitude, self.crew_size
        )
    }
}
fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };
    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254,
    };
    println!("hubble is {}", hubble.describe());
    println!("iss is {}", iss.describe())
}
