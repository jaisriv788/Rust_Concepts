#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    let ok: Result<i32, &str> = Result::Ok(32);
    let err: Result<i32, &str> = Result::Err("Error their!");

    println!("Basics -> {:?}", ok);
    // println!("{:?}", err.unwrap());

    //Real Wrold Example of result enum
    let str = "54";
    //turbo fish operator is complusary to get infered types of the result enum.
    let string_to_number = str.parse::<i16>();
    println!("Parse Method -> {:?}", string_to_number);

    //calling the divide funtion
    println!("Divide funtion -> {:?}", divide(12.0, 3.0));
    println!("Divide funtion -> {:?}", divide(0.0, 12.0));
    println!("Divide funtion -> {:?}", divide(0.0, 12.0).is_err());
    println!("Divide funtion -> {:?}", divide(0.0, 12.0).is_ok());
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    let ans = numerator / denominator;
    match ans {
        0.0 => Result::Err(String::from("Not Possble")),
        _ => Result::Ok(ans),
    }
}
