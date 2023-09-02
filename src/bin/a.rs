fn main() {
    proconio::input! {
        n: u32,
        m: u32,
        p: u32
    }
    println!("{}", if m > n {0} else {1 + (n - m) / p});
}
