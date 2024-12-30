mod generics_in_enum;

use std::any::type_name;

#[derive(Debug)]
struct TreasureChest<T, U> {
    owner: T,
    founder: T,
    amount: U,
}

struct Money<T> {
    currency: String,
    amount: T,
}

impl TreasureChest<String, i32> {
    fn result(&self) {
        println!("This will work for specific type.");
        println!("owner: {}", self.owner);
    }
}

impl<T: std::fmt::Display, U: std::fmt::Display> TreasureChest<T, U> {
    fn res(&self) {
        println!(
            "This will work for every type and the owner is {} {}",
            self.owner.to_string().to_uppercase(),
            self.amount
        );
    }
}

impl Money<i32> {
    fn conv(&self) {
        println!("This will work for specific type.");
        println!("owner: {}", self.currency);
    }
}

impl<T: std::fmt::Display> Money<T> {
    fn converter(&self) {
        println!(
            "Money converter called! {} {}",
            self.amount,
            self.currency.to_uppercase()
        );
    }
}

fn main() {
    // Single Generic Parameter
    println!("SINGLE GENERIC PARAMETER -> ");
    println!("{}", identity::<i16>(32));
    println!("{}", identity::<bool>(true));
    println!("{}", identity::<String>(String::from("hello world")));
    println!("-------------------------------------------------------\n");

    // Multiple Generic Parameter
    println!("MULTIPLE GENERIC PARAMETER -> ");
    println!("{:?}", tuple::<String, i32>(String::from("Beep"), 28));
    println!("-------------------------------------------------------\n");

    //Generics in struct with single parameter
    println!("GENERIC IN STRUCT SINGLE PARAMETER -> ");
    let old_treasure: Money<i32> = Money {
        currency: String::from("Rupees"),
        amount: 30000,
    };
    old_treasure.converter();
    old_treasure.conv();
    //But if the type is changed in the above code then the above
    //then the conv method will not work
    println!("-------------------------------------------------------\n");

    //Generics in struct with multiple parameter
    println!("GENERIC IN STRUCT MULTIPLE PARAMETER -> ");
    let new_treasure: TreasureChest<String, i32> = TreasureChest {
        owner: String::from("Jai"),
        founder: String::from("jama"),
        amount: 32000000,
    };
    println!("{:?}", new_treasure);
    println!("-------------------------------------------------------\n");

    //Implementation of function with generics
    println!("GENERIC IN METHODS -> ");
    let _treasure: TreasureChest<String, u32> = TreasureChest {
        owner: String::from("Jaya"),
        founder: String::from("jamaamama"),
        amount: 32000000,
    };

    new_treasure.result();
    //below code will not work as the type do not match with the impl conditions
    // _treasure.result();
    //but below both will work
    new_treasure.res();
    _treasure.res();
    println!("-------------------------------------------------------\n");

    //----------------------------------------------------------------------

    // Print the data type of each field
    println!("DATA TYPES -> ");
    println!("Type of owner: {}", type_of(&new_treasure.owner));
    println!("Type of founder: {}", type_of(&new_treasure.founder));
    println!("Type of amount: {}", type_of(&new_treasure.amount));

}

// Helper function to get the typeof a value
fn type_of<T>(_: &T) -> &str {
    type_name::<T>()
}

// Single Generic Parameter
fn identity<T>(val: T) -> T {
    val
}

// Multiple Generic Parameter
fn tuple<T, U>(val_1: T, val_2: U) -> (T, U) {
    (val_1, val_2)
}
