enum Milk {
    Lowfat(i32),
    Whole,
}

impl Milk {
    fn drink(self) {
        match self {
            //below will only run if the low fat will have the same number within the parenthesis
            Milk::Lowfat(2) => println!("Lowfat"),
            Milk::Whole => println!("Whole"),
            _ => println!("Not good"),
            // Milk::Lowfat(other) => println!("Lowfat {other:?}"),
        }
    }
}

fn main() {
    Milk::Lowfat(2).drink();
    Milk::Lowfat(3).drink();
    Milk::Whole.drink();
}
