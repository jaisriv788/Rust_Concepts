fn main() {
    let is_concert: bool = false;
    let is_event = is_concert;
    println!("In bool {is_concert} {is_event}");

    let sushi= "Salmon";
    let dinner = sushi;
    println!("In string literal {dinner} {sushi}");

    let sushi1: String = String::from("salmon");
    let dinner1: String = sushi1;
    //here ownership is transferred
    // println!("{dinner1} {sushi1}");

    let result = eat_meal(dinner1);
    println!("this is the result {result} here.");
}

fn eat_meal(mut meal: String) -> String{
    meal.clear();
    meal
}