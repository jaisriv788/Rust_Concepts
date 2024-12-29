
#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivering,
    Delivered,
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            OnlineOrderStatus::Ordered => {
                println!("You placed your order.");
            }
            OnlineOrderStatus::Packed | OnlineOrderStatus::Shipped => {
                println!("Your package is packed and ready to move.");
            }
            //below on is a solution for the rest of the methods
            // _ => {
            //     println!("Package on the move to be delivered.");
            // }
            //option to for the above commented code is ->
            other => {
                println!("The order is been {other:?}.")
            }
        }
    }
}

fn main() {
    // For the ordered enum this is the call
    OnlineOrderStatus::Ordered.check();

    //For the rest of the option below is the call
    OnlineOrderStatus::Packed.check();
    OnlineOrderStatus::Shipped.check();
    OnlineOrderStatus::Delivering.check();
    OnlineOrderStatus::Delivered.check();
}
