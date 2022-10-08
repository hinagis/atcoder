fn main() {
    proconio::input! {
        n: usize,
        a: [u32; n]
    }
    println!("{}", a.iter().fold(0, |s, a| s + a));
}
