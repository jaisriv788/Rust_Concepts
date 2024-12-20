fn main() {
    apply_to_jobs(7, "Web Developer");

    let res = is_even(8);
    println!("{res}");
    dbg!(is_even(9));

    dbg!(alphabets("jai"));
    dbg!(alphabets("hello"));
    dbg!(alphabets("zebra"));
    dbg!(alphabets("zoo"));
}

fn apply_to_jobs(number: i32, title: &str) {
    println!("I'm applying to {} {} jobs.", number, title);
}

fn is_even(number: i32) -> bool {
     number % 2 == 0 
}

fn alphabets(text: &str) -> (bool, bool) {
    (text.contains('a'), text.contains('z'))
}
