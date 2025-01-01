#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    let present_value: Option<i32> = Option::Some(13);
    let missing_value: Option<i32> = Option::None;

    println!(
        "{}\n{}",
        present_value.unwrap_or(22),
        missing_value.unwrap_or(22)
    );
}
