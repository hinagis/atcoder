fn main() {
    proconio::input! {
        x: u32,
        c: u32
    }
    println!("{}", (x / (1000 + c)) * 1000);
}
