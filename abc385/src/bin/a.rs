fn main() {
    proconio::input! {
        a: u32,
        b: u32,
        c: u32
    }
    println!("{}", if a == b && b == c || a + b == c || a == b + c || a + c == b { "Yes" } else { "No" });
}
