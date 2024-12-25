fn main() {
    let mut food: String = String::from("food");
    add_item(&mut food);
    show_food(&food);
    println!("{}", food);
}

fn add_item(food: &mut String){
    food.push_str(" is nice.")
}

fn show_food(food: &String){
    println!("{}", food)
}
