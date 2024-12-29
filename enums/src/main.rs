#![allow(dead_code)]

mod enums_in_enums;
mod match_in_enums;

#[derive(Debug)]
enum WeekDays {
    Monday,
    Tuesday,
    Wednesday(String),
    Thursday {
        destination: String,
        budget: i32,
        distance: i32,
    },
    Friday,
    Saturday(i32, String),
    Sunday(HolidayPlan),
}
#[derive(Debug)]
struct HolidayPlan {
    destination: String,
    budget: i32,
    distance: i32,
}

fn main() {
    //Classic enum calling
    let first_day = WeekDays::Monday;
    println!("{:?}", first_day);
    //making a tuple with enum
    let work_day = (WeekDays::Tuesday, String::from("Coding"));
    println!("{:?}", work_day);
    //assigning related data to enum as specified in the enum declaration
    let new_way = WeekDays::Wednesday(String::from("Testing"));
    let spent_day = WeekDays::Saturday(3000, String::from("Clothes"));
    println!("{:?}", new_way);
    println!("{:?}", spent_day);

    //assigning related data to enum as specified in the enum declaration as Struct
    let planning = HolidayPlan {
        destination: String::from("Gokarna"),
        budget: 3000,
        distance: 325,
    };

    let holiday = WeekDays::Sunday(planning);
    println!("{:?}", holiday);

    //using struct directly into the enum
    let new_planning = WeekDays::Thursday {
        destination: String::from("Bali"),
        budget: 6000,
        distance: 3125,
    };
    println!("{:?}", new_planning);
}
