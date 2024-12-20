fn main() {
    let season: &str = "spring";

    if season.to_lowercase() == "summer" {
        println!("{} season started!", season.to_lowercase());
    } else if season.to_lowercase() == "winter" {
        println!("{} season ended!", season.to_lowercase());
    } else if season.to_lowercase() == "spring" {
        println!("{} season ended!", season.to_lowercase());
    } else if season.to_lowercase() == "rain" {
        println!("{} season ended!", season.to_lowercase());
    } else {
        println!("entered season does not exist");
    }

    println!("-------------------------------");

    let res = is_even(5);
    dbg!(res);

    println!("-------------------------------");

    //Below is a replacement in rust as switch is not their in rust so it's replaced by Match
    let is_present = false;

    match is_present {
        true => {
            println!("is present :)");
        }
        false => {
            println!("is not present :)");
        }
    }

    println!("-------------------------------");

    //other way to use it is
    let result = match is_present {
        true => 10,
        false => 20,
    };
    dbg!(result);

    println!("-------------------------------");

    //In case of sting match will have infinite number of cases so to manage that we could do the following
    //In it the sequence matters.
    match season {
        "summer" => println!("{} season started!", season.to_lowercase()),
        "winter" => println!("{} season ended!", season.to_lowercase()),
        _ => println!("entered season does not exist"),
    }

    println!("-------------------------------");

    //For multiple values we use it like
    let num = 8;
    match num {
        2 | 4 | 6 | 8 => println!("Even"),
        1 | 3 | 5 | 7 => println!("Odd"),
        _ => println!("Unknown"),
    }

    println!("-------------------------------");

    match num {
        value if value % 2 == 0 => println!("even number"),
        value if value % 2 != 0 => println!("odd number"),
        _ => unreachable!(),
    }

    println!("-------------------------------");

    //Loop
    let mut loop_var: i32 = 10;
    loop {
        if loop_var == 0 {
            break;
        }
        println!("loop var is {}", loop_var);
        loop_var -= 1;
    }

    println!("-------------------------------");

    while loop_var <= 10 {
        loop_var += 1;
        println!("loop var is {}", loop_var);
    }

    println!("-------------------------------");

    recursion(5);
}

fn is_even(num: i32) -> bool {
    let res: bool = if num % 2 == 0 { true } else { false };
    res
}

fn recursion(seconds: i32) {
    //unlike loops recursion have a base to get terminated. like as follows.
    if seconds == 0 {
        //This is a base case.
        println!("Time Out.");
    } else {
        println!("{seconds}seconds to go.");
        recursion(seconds - 1);
    }

}
