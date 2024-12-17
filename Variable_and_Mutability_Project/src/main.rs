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

    //Down below is a compiler directive and can we used over next line or function or on the whole file depending upon the usage.
    //But to use over whole file it must be written at the top of the file with a minor change i.e. #![allow(unused_variables)]
    #[allow(unused_variables)] 
     let favourite_food : &str = "Chicken";

}