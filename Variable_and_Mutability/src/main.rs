fn main() {
    //VARIABLES
    //This is how we decalre a variable in RUST.
    let assumed_value = 60;
    let expected_value = 21 + 3;
    let result = (assumed_value + expected_value) / 2;

    //If you want a variable to be unused and you do not want a warning than we should decalre it like below
    let _unused = 23;

    //To print dynamic reult we can do the following
    println!("So the result is {result}");
    println!(
        "We took the assumed value = {} and expected_value = {}",
        assumed_value, expected_value
    );

    //-------------------------------------------------------------------------------------------------------

    //MUTABILITY
    //Their are two types of mutability for varibales :- 1) Immutable  2) Mutable
    //By default the variables are Immutable

    //1)Immutable variable
    let value1 = 34;
    println!("Immutable variable value and name is => value1 = {value1}");
    //The below code of line will give you error as it not possible because the variable is immutable
    // value1 = 22;

    //Mutable variable
    let mut value2 = 34;
    println!("Mutable variable value and name is => value2 = {value2}");
    value2 = 35;
    println!("Mutable variable value and name after mutating is => value2 = {value2}");
}
