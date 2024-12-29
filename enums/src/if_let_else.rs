#[derive(Debug)]
enum Dairy {
    Whole(u32),
    NonDairy { kind: String },
}

fn main() {
    //if let ->
    let my_product = Dairy::Whole(95);
    //code below says that if the my_product variable is a Whole from Dairy
    //enum than only let the code in th curly brackets run otherwise do not.
    if let Dairy::Whole(percentage) = my_product {
        println!("{percentage}% Whole Milk.");
    }

    let my_new_product = Dairy::NonDairy {
        kind: String::from("Boba"),
    };
    if let Dairy::NonDairy { kind } = my_new_product {
        println!("{kind}");
    } else {
        println!("Dairy");
    }

    //let else
    let product = Dairy::Whole(2);
    //Code below says that if the type do not match the variable type then only
    //run the block and have the variable declared.

    //BELOW THE ELSE CODE WILL EXECUTE
    // let Dairy::NonDairy{kind} = product else {
    //     println!("Condition do not match");
    //     return;
    // };
    // println!{"{kind}"};

    let Dairy::Whole(percent) = product else {
        println!("Condition do not match");
        return;
    };
    println! {"{percent}"};
}
