fn main() {
    proconio::input! {
        n: usize,
        s: u32,
        t: [u32; n]
    }
    println!("{}", if t[0] <= s && (1..n).all(|i| t[i] - t[i - 1] <= s) { "Yes" } else { "No" });
}
