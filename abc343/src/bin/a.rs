fn main() {
    proconio::input! {
        a: u32,
        b: u32,
    }
    println!("{}", if a + b == 0 {1} else {0});
}
