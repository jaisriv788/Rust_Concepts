#![allow(dead_code)]

fn main() {
    let result = in_stock(true, true);
    println!("{:?}", result);
    let result = in_stock(true, false);
    println!("{:?}", result);
    let result = in_stock(false, false);
    println!("{:?}", result);
}

fn in_stock(in_system: bool, in_stock: bool) -> Option<bool> {
    if in_stock && in_system {
        Option::Some(true)
    } else if in_system {
        Option::Some(false)
    } else {
        Option::None
    }
}
