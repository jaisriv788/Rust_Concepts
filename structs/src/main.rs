#[derive(Debug)] //this line allows to print the whole object or struct in a single line.
struct Details {
    name: String,
    age: i16,
    gender: String,
    is_black: bool,
}

fn main() {
    //Structs are just like objects in any other language these are of 3 types ->
    //1) Named field structs 2) Tuple like structs 3) Unit like structs

    let mut jai = Details {
        name: String::from("Jai"),
        age: 24,
        gender: String::from("Male"),
        is_black: false,
    };

    println!("{} {} {} {}", jai.name, jai.age, jai.gender, jai.is_black);

    jai.name = String::from("Jaya");
    jai.age = 38;
    jai.gender = String::from("Female");

    println!("{} {} {} {}", jai.name, jai.age, jai.gender, jai.is_black);

    let abhay_details: Details =
        make_details(String::from("Abhay"), 32, String::from("Male"), true);

    println!("{:?}", abhay_details);

    //If you want to create a new struct instance and have some of the property similar to the already declared one
    //then we use spread operator like below

    let depansh_details: Details = Details {
        name: String::from("Depansh"),
        age: 44,
        ..abhay_details
    };
    println!("{:?}", depansh_details);

    let new_person_one = make_details(String::from("Hero"), 44, String::from("Male"), false);
    let new_person_two = make_details(String::from("Hero"), 44, String::from("Male"), false);
    let new_person_three = make_details(String::from("Hero"), 44, String::from("Male"), false);
    let mut new_person_four = make_details(String::from("Hero"), 44, String::from("Male"), false);

    //There are four different ways to pass struct into function
    type_one(new_person_one); //immutable ownership is being passed of new_person_one inside the type on function type_one
    //new_person_one.age=43;  It will give error because its ownership is being passed on inside the parameter of the function
    type_two(new_person_two); //immutable ownership is being passed of new_person_two inside the type on function type_two but in the parameter is being changed to mutable
    //new_person_two.age=43;  It will give error because its ownership is being passed on inside the parameter of the function
    type_three(&new_person_three); //immutable referencing is passed on of new_person_three in type_three function
    //new_person_three.age=43; It's giving error because its immutable, but its ownership is not being passed on to the function
    type_four(&mut new_person_four); //mutable referencing is passed on of new_person_four in type_four function
    new_person_four.age = 43; //It's not giving error as it's a mutable variable and its ownership is not being passed to the function
}

fn type_one(details: Details) {
    println!("type 1 ->{} {}", details.name, details.age);
}
fn type_two(mut details: Details) {
    println!("type 2 -> {} {}", details.name, details.is_black);
    details.is_black = true;
    println!("type 2 -> {} {}", details.name, details.is_black);
}
fn type_three(details: &Details) {
    println!("type 3 -> {} {}", details.name, details.age);
    //below line will throw error as its reference is immutable
    //details.is_black = true;
}
fn type_four(details: &mut Details) {
    println!("type 4 ->{} {}", details.name, details.is_black);
    details.is_black = true;
    println!("type 4 ->{} {}", details.name, details.is_black);
}

fn make_details(name: String, age: i16, gender: String, is_black: bool) -> Details {
    Details {
        name,
        age,
        gender,
        is_black,
    }
}
