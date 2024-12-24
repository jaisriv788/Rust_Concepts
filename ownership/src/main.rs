use std::io::stdin;

#[allow(unused_variables)]
fn main() {
    let mut text: String = String::new();
    // The line `stdin().read_line(&mut text).unwrap();` is reading input from the standard input (stdin)
    // and storing it in the mutable variable `text`. The `read_line` method reads a line from the input
    // and appends it to the provided `String` variable. The `&mut text` part is passing a mutable
    // reference to the `text` variable, allowing the `read_line` method to modify the variable directly.
    // The `unwrap` method is used to handle any potential errors that may occur during the reading
    // process.
    stdin().read_line(&mut text).unwrap();
    println!("{text}");

    let mut name: String = String::from("Jai");
    name.push_str(" Srivastava");

    println!("{name}");

    let var = String::from("hello");
    drop(var);
    //the below line is not possible as its value or the reference is dropped out
    // println!("{var}");

    // The code snippet is creating two String variables `var1` and `var2`.
    let var1 = String::from("hello");
    // `let var2 = var1.clone();` is creating a deep copy of the `String` variable `var1`. This means
    // that a new memory allocation is made for `var2` and the content of `var1` is copied into it. This
    // ensures that `var1` and `var2` are completely independent of each other and modifying one will
    // not affect the other.
    let var2 = var1.clone();
    println!("{var1}\n{var2}");

    //Referencing and borrowing
    let my_val = 2;
    // `let my_ref = &my_val;` is creating a reference to the variable `my_val`. In Rust, the `&` symbol is
    // used to create references to values rather than making a copy of the value itself.
    let my_ref = &my_val; //reference operator ==> &

    let my_heap = String::from("hello heap");
    let my_heap_ref = &my_heap;

    //Deference Operator ==> *
    println!("{}", *my_heap_ref);

    let value = "devi";

    //Ownership and function parameter
    //for stack data
    let apples = 32;
    print_my_val(apples);
    println!("In the main function{}", apples);

    //for heap data
    let mango = String::from("mango");
    print_my_val2(mango);

    //rust can not be used after being passed as it does not have copy trait.
    // println!("In the main function{}",mango);

    let fruits = "peter";
    print_my_val3(fruits);
    println!("{}", fruits);

    //Mutable parameters
    let food = String::from("food");
    add_food(food);
}

fn add_food(mut food: String) {
    food.push_str(" is good.");
    println!("{}", food);
}

fn print_my_val3(value: &str) {
    println!("{}", value);
}
fn print_my_val2(value: String) {
    println!("{value}");
}

fn print_my_val(val: i32) {
    println!("{}", val);
}
