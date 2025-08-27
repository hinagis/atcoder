fn main() {
    proconio::input! {
        x: u8,
        y: u8
    }
    println!("{}", (x + y - 1) % 12 + 1);
}
