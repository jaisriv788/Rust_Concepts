// `#[allow(unused_variables)]` is a compiler directive in Rust that suppresses warnings related to
// unused variables. This attribute tells the Rust compiler to ignore warnings that would typically be
// generated when variables are declared but not used in the code. It can be helpful during development
// to avoid unnecessary noise from the compiler about variables that are intentionally left unused.
#[allow(unused_variables)]
fn main() {
    //Below are the decalration for the data types.

    //A)Integer
    //These are of two types ->
    //1) Signed Integer
    let eight_bit_int: i8 = 28;
    let sixteen_bit_int: i16 = 302;
    let thirtytwo_bit_int: i32 = 54664;
    let sixtyfour_bit_int: i64 = 54664878;
    let onetwentyeight_bit_int: i128 = 54664878656;

    //2)Unsigned Integer
    let un_eight_bit_int: u8 = 28;
    let un_sixteen_bit_int: u16 = 302;
    let un_thirtytwo_bit_int: u32 = 54664;
    let un_sixtyfour_bit_int: u64 = 54664878;
    let un_onetwentyeight_bit_int: u128 = 54_66_48_78_656;

    //The below two are some what similar but depends up the system u are using
    let num1: isize = 40003;
    let num2: usize = 32432;

    //B)Float
    let pi1: f32 = 22.0 / 7.0;
    let pi2: f64 = 22.0 / 7.0;

    //to control the number of digits after the decimal we use somthing like below
    // The code `println!("{pi1:.2}"); println!("{:.3}", pi2);` is formatting and printing the
    // floating-point numbers `pi1` and `pi2` with a specific number of decimal places.
    println!("{pi1:.2}");
    println!("{:.3}", pi2);

    //Type Casting
    let cast: f32 = 456.756;
    let cast1 = cast as i32;

    println!("{cast1}");

    //C)Boolean
    let is_valid: bool = true;
    let is_done: bool = false;

    //D)Characters
    let char1: char = 'a';
    let char2: char = '😊';

    println!("{char2}");

    //Array
    let arr: [i32; 6] = [1, 2, 3, 4, 5, 6];
    //below line is not executable as it does not contain display trait but have a debug trait.
    // println!("{}",arr);

    // `println!("{:?}", arr);` is a Rust macro that is used to print the debug representation of the
    // `arr` array. The `{:?}` format specifier is used to print the array in a debug format, which
    // means it will display the content of the array in a way that is suitable for debugging purposes.
    // This format specifier is commonly used to print arrays, structs, enums, and other complex data
    // types for debugging and inspection.
    println!("{:?}", arr);
    println!("{arr:?}");

    // `println!("{:#?}", arr);` is a Rust macro that is used to print the debug representation of the
    // `arr` array with a pretty-printing format. The `{:?}` format specifier is used to print the
    // array in a debug format, and the `#` symbol modifies the output to be more visually structured
    // and easier to read. This format specifier is commonly used for debugging purposes to display
    // complex data types like arrays, structs, enums, etc.
    println!("{:#?}", arr);
}
