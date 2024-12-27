use std::io;

fn main() {
    let mut string = String::new();

    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read line");

    let integer: i32 = string.trim().parse().expect("Not a number!");

    println!("{}", string);
    println!("{}", integer);
}
