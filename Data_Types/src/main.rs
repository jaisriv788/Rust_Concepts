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
}
