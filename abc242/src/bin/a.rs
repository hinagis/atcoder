fn main() {
    proconio::input! {
        a: u32,
        b: u32,
        c: f64,
        x: u32,
    }

    if x <= a {
        println!("1");
    } else if x > b {
        println!("0");
    } else {
        println!("{}", c / (b - a) as f64);
    }
}
