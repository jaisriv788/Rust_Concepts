#[allow(dead_code)]
#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f32,
    passenger: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f32, passenger: u32) -> Self {
        Self {
            origin,
            destination,
            price,
            passenger,
        }
    }
}

impl Flight {
    fn change_destination(&mut self, destination: String) -> &mut Self {
        self.destination = destination;
        self
    }

    fn increase_price(&mut self) -> &mut Self {
        self.price *= 1.2;
        self
    }

    fn itinerary(&self) {
        println!("{} -> {}", self.origin, self.destination);
    }
}

fn main() {
    let mut mumbai = Flight::new(
        String::from("Mumbai"),
        String::from("Banglore"),
        5900.99,
        112,
    );
    let banglore = Flight {
        origin: String::from("Banglore"),
        destination: String::from("Mumbai"),
        ..mumbai
    };
    mumbai
        .change_destination(String::from("Chennai"))
        .increase_price();

    mumbai.itinerary();
    println!("{:?} \n",mumbai);

    banglore.itinerary();
    println!("{:?}",banglore);
}
