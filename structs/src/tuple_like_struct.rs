struct ShortDuration(u32,u32);

struct LongDuration(u32,u32);

//unit like struct
struct Empty;

fn main() {
    let work_shift = ShortDuration(8,30);
    println!("{} hours {} mins", work_shift.0,work_shift.1);
    let work_long = LongDuration(10,20);
    println!("{} years {} months", work_long.0,work_long.1);
}