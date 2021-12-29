fn main() {
    proconio::input! {
        x: u32,
        y: u32,
    }
    println!("{}", if x >= y {0} else {(y - x + 9) / 10});
}
