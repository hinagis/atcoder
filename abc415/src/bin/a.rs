fn main() {
    proconio::input! {
        a: [u8],
        x: u8
    }
    println!("{}", if a.contains(&x) { "Yes" } else { "No" });
}
