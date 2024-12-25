fn main() {
    proconio::input! {
        n: u32,
        x: u32,
        t: u32,
    }

    println!("{}", ((n - 1) / x + 1) * t);
}
