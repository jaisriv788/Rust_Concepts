fn main() {
    let res1: i32 = color_to_number("blue");
    println!("res1: {}", res1);

    let res2 = factorial_without_recursion(5);
    println!("res2: {}", res2);

    let res3: i32 = factorial_with_recursion(5);
    println!("res3: {}", res3);
}

fn factorial_without_recursion(mut num: i32) -> i32 {
    let mut ans = 1;
    loop {
        if num == 1 {
            break;
        }
        ans = ans * num;
        num -= 1;
    }
    ans
}

fn factorial_with_recursion(num: i32) -> i32 {
    if num == 1{
       return 1;
    }

    num * factorial_with_recursion(num-1)
}

fn color_to_number(color: &str) -> i32 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
    // if color == "red" {
    //     1
    // } else if color == "green" {
    //     2
    // } else if color == "blue" {
    //     3
    // } else {
    //     0
    // }
}
