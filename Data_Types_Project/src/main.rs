#[allow(unused_variables)]
fn main() {
    let var_1: i32 = 1_337;
    let var_2 = var_1 as i16;

    let var_3: f64 = 3.1456734;
    println!("{var_3:.3}");

    let with_milk: bool = true;
    let with_sugar: bool = false;
    let is_my_type_of_coffee: bool = with_milk && with_sugar;
    let is_acceptable_coffee: bool = with_milk || with_sugar;

    let arr: [i8; 4] = [1, 2, 3, 4];
    dbg!(arr);

    let tup: (i32, f32, bool, [i8; 4]) = (28, 3.14, false, arr);
    dbg!(tup);
}
