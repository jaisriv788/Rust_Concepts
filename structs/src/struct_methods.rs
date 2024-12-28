#[derive(Debug)]
struct DetailsNew {
    name: String,
    age: i16,
    gender: String,
    is_black: bool,
}

//below is the method to assign methods to the struct.
//We can implement multiple instances of impl for a struct.
impl DetailsNew {
    fn new(name: String, age: i16, gender: String, is_black: bool) -> DetailsNew {
        DetailsNew{
            name,
            age,
            gender,
            is_black
        }
    }
}
impl DetailsNew {
    //immutable self and here ownership will be passed on to here
    fn display_details_immutable_self(self) {
        println!("name is {}", self.name);
    }
    //mutable self and here ownership will be passed on to here
    fn display_details_mutable_self(self) {
        println!("age is {}", self.age);
    }
    //immutable reference of self and here ownership will not be passed on to here
    fn display_details_immut_ref(&self) {
        println!("gender is {}", self.gender);
    }
    //immutable reference of self and here ownership will not be passed on to here
    fn display_details_mut_ref(&self) {
        println!("It is {} that person is black.", self.is_black);
    }

    //methods with multiple parameter
    fn multiple_parameter(&self, city: String, year: i32) {
        println!(
            "The name of the person is {} and he lives in {} since {}",
            self.name, city, year
        );
    }
}

fn main() {

    let hero = DetailsNew::new(String::from("Arnav"), 44, String::from("Male"), false);
    //hero.display_details_immutable_self(); It will move the ownership and hence it cannot be passed on to the next line so it will give error
    //hero.display_details_mutable_self(); It will move the ownership and hence it cannot be passed on to the next line so it will give error
    hero.display_details_immut_ref();
    hero.display_details_mut_ref();
    hero.multiple_parameter(String::from("Lucknow"), 2007);

    let new_hero = make_details(String::from("Arnav"), 44, String::from("Male"), false);
}

fn make_details(name: String, age: i16, gender: String, is_black: bool) -> DetailsNew {
    DetailsNew {
        name,
        age,
        gender,
        is_black,
    }
}