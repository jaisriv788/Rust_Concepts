#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
enum MyOption {
    Some(i32),
    None,
}

impl MyOption {
    fn unwrap(self) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Value not present"),
        }
    }

    fn except(self, fallback_error: &str) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("{}", fallback_error),
        }
    }

    fn unwrap_or(self, fallback_value: i32) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value,
        }
    }
}

pub fn main() {
    let some = MyOption::Some(2);
    let none = MyOption::None;

    println!("1.{some:?} => {}", some.unwrap());
    // println!("2.{none:?} => {}", none.unwrap());
    println!("3.{some:?} => {}", some.unwrap_or(21));
    println!("4.{none:?} => {}", none.unwrap_or(32));

    //self made
    println!("5.{none:?} => {}", some.except("Something Went Wrong."));
    println!("6.{none:?} => {}", none.except("Something Went Wrong."));
}
