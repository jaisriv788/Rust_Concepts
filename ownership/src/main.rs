use std::io::stdin;
fn main() {
    let mut text: String = String::new();
    stdin().read_line(&mut text).unwrap();
    println!("{text}");

    let mut name: String = String::from("Jai");
    name.push_str(" Srivastava");

    println!("{name}");

    let var = String::from("hello");
    drop(var);
    //the below line is not possible as its value or the reference is dropped out
    // println!("{var}");

    let var1 = String::from("hello");
    let var2 = var1.clone();
    println!("{var1}\n{var2}");
}
