#![allow(dead_code)]

#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        match self.has_mice_infestation {
            true => Option::None,
            false => match self.reservations {
                value if value < 12 => Option::Some(Food {
                    name: String::from("Uni Sashimi"),
                }),
                value if value >= 12 => Option::Some(Food {
                    name: String::from("Strip Steak"),
                }),
                _ => Option::None,
            },
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            Result::Err(String::from("Sorry, we have a mice problem."))
        } else if address.is_empty() {
            Err(String::from("No delivery address specified"))
        } else {
            Result::Ok(Food {
                name: String::from("Burger"),
            })
        }
    }
}

fn main() {
    let restaurant_1 = Restaurant {
        reservations: 11,
        has_mice_infestation: false,
    };
    let restaurant_2 = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };
    let restaurant_3 = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };

    //for checf_special function
    println!("{:?}", restaurant_1.chef_special());
    println!("{:?}", restaurant_2.chef_special());
    println!("{:?}", restaurant_3.chef_special());

    //for deliver_burger funciton
    println!("{:?}", restaurant_2.deliver_burger("9/96a jankipuram extension."));
    println!("{:?}", restaurant_1.deliver_burger(""));
    println!("{:?}", restaurant_1.deliver_burger("9/96a jankipuram extension."));
}
