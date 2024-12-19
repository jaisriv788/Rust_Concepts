fn main() {
    open_store("9:00");
    send_profit(500);
    close_store("8:00");

    let result = square(4);
    dbg!(result);

    let res = {
        let cube: i32 = 3;
        cube.pow(3)
    };
    dbg!(res);
}

fn open_store(time: &str) {
    println!("open store at {time}am");
}
fn close_store(time: &str) {
    println!("close store at {time}pm");
}
fn send_profit(money: i32) {
    println!("send profit of ${money}");
}
//return type function
fn square(num: i32) -> i32 {
    num * num
    //return num * num;
}
