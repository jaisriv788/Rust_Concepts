
mod returning_option_from_function;
mod unwrap_or;
mod my_own_option;
mod result_enum;

fn main() {
    //Below are two varients of inbuilt Option enum and how to use them as they have oonly two values
    //1)None 2)Some(T)
    //let _x: Option<String> = Option::<String>::Some(String::from("Jai"));
    //let _b: Option<i32> = Option::None;

    //How to use Option in Real World =>

    let arr: [String; 3] = [
        String::from("Mysuru"),
        String::from("Delhi"),
        String::from("Lucknow"),
    ];

    let arr_res_1: Option<&String> = arr.get(0);
    let arr_res_1a: Option<&String> = arr.get(3);
    // let arr_res_2 = arr.get(2).unwrap();
    // let arr_res_3: Option<&String> = arr.get(3);

    //below print hasve unwrap method that works on Option enum but the prob is that
    //it works fine with Some but it throw err or panics as it gets None
    // println!(
    //     "Option::Some(T) => {:?} \nOption::Some(T) => {:?} \nOption::Some(T) => {:?} \nOption::None => {:?}",
    //     arr_res_1,
    //     arr_res_1.unwrap(),
    //     arr_res_2,
    //     arr_res_3
    // );

    //So to resolve the above error we have expect method it will still give us error but
    //with a custom error message it help us to debug the code
    // println!(
    //     "Option with expect methods => {}\nOption with expect methods with error => {}",
    //     arr_res_1.expect("Wrong index"),
    //     arr_res_1a.expect("Wrong index")
    // );
    options(arr_res_1);
    options(arr_res_1a);
}

fn options(value: Option<&String>) {
    match value {
        Option::Some(val) => {
            println!("{}", val);
        }
        Option::None => {
            println!("Wrong Index");
        }
    }
}
