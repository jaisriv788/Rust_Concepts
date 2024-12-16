const TOUCHDOWN_POINTS: i32 = 6;

fn main() {
    let season : &str = "October";
    let mut points_scored: i32 = 28;
    println!("{}",points_scored);
    points_scored = 35;

    let event_time : &str = "06:00";
    println!("{0}",event_time);
    let event_time : i32 = 6;

    println!("{season} {points_scored} {event_time} {TOUCHDOWN_POINTS}");

    let _favourite_beverage : &str = "Coco Cola";

    #[allow(unused_variables)]
     let favourite_food : &str = "Chicken";

}