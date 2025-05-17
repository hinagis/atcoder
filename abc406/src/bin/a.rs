fn main() {
    proconio::input! {
        a: u32,
        b: u32,
        c: u32,
        d: u32,
    }
    println!("{}", if c < a || a == c && d < b { "Yes" } else { "No" });
}
