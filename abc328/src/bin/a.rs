fn main() {
    proconio::input! {
        n: usize,
        x: u32,
        s: [u32; n]
    }
    println!("{}", s.iter().filter(|&&s| s <= x).fold(0, |s, e| s + e));
}
