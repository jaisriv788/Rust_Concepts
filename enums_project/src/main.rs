#![allow(dead_code)]

#[derive(Debug)]
enum Tire {
    Gold(u32),
    Silver(u32),
    Platinum(u32),
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium { tire: Tire },
}

impl Subscription {
    fn summarize(&self) {
        match self {
            Subscription::Free => {
                println!("You have limited access to the website.")
            }
            Subscription::Basic(price, month) => {
                println!("You have limited access to the site's premium features for {price} for {month} months")
            }
            Subscription::Premium { tire } => {
                match tire {
                    Tire::Gold(months) => {
                        println!("You are now a Gold member for {months}");
                    }
                    Tire::Silver(months) => {
                        println!("You are now a Silver member for {months}");
                    }
                    Tire::Platinum(months) => {
                        println!("You are now a Platinum member for {months}");
                    }
                }
            }
        }
    }
}

fn main() {
    //For Free
    Subscription::Free.summarize();

    //For Basic
    Subscription::Basic(499.99,12).summarize();

    //For Premium
    Subscription::Premium { tire: Tire::Silver(6) }.summarize();
    Subscription::Premium { tire: Tire::Gold(12) }.summarize();
    Subscription::Premium { tire: Tire::Platinum(18) }.summarize();
}
