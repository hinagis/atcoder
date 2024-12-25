fn main() {
    proconio::input! {
        _: u32,
        m: u32,
        x: u32,
        t: u32,
        d: u32,
    }
    let a = t - d * x;
    println!("{}", if m < x {a + d * m} else {t});
}
