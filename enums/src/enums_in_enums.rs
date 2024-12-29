#![allow(dead_code)]

#[derive(Debug)]
enum Menu {
    VegThali(VegThali),
    NonVegThali(NonVegThali),
    ChineseThali { veg: VegThali, non_veg: NonVegThali },
}

#[derive(Debug)]
enum VegThali {
    Normal,
    Special,
}

#[derive(Debug)]
enum NonVegThali {
    Chicken,
    Mutton,
}

fn main() {
    let jai = Menu::NonVegThali(NonVegThali::Chicken);
    println!("Jai will have ->{:?}", jai);

    let devyanshi = Menu::VegThali(VegThali::Special);
    println!("Devyanshi will have ->{:?}", devyanshi);

    let ashish = Menu::ChineseThali {
        veg: VegThali::Normal,
        non_veg: NonVegThali::Mutton,
    };
    println!("Ashish will have ->{:?}", ashish);
}
