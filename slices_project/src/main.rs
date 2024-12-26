fn main() {
    let mut arr: [String;5] = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch"),
    ];

    let first_two = &arr[..2];
    println!("{first_two:?}");

    let mid_three = &arr[1..4];
    println!("{mid_three:?}");

    let last_three = &mut arr[2..];
    println!("{last_three:?}");

    last_three[2] = String::from("Lucky Charms");
    println!("{arr:?}");

    let cookie_crisp = &arr[0];
    let cookie = &cookie_crisp[0..6];
    println!("{}",cookie);

    let cocoa_puffs = &arr[3];
    let puff = &cocoa_puffs[6..];
    println!("{}",puff);
}
