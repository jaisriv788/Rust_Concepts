#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
enum Cheese<T> {
    Plain,
    Topping(T),
}

fn main() {
    let mushroom = Cheese::Topping("mushroom");
    let onion = Cheese::Topping(String::from("onion"));
    let chicken = Cheese::Topping(&String::from("chicken"));

    //Whenever the option in enum does not have a type defined then also we have to 
    //define the type to the variable explictl like below.
    let plain: Cheese<String> = Cheese::Plain;
}
