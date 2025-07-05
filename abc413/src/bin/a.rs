fn main() {
    proconio::input! {
        n: usize,
        m: u32,
        a: [u32; n]
    }
    println!("{}", if a.iter().sum::<u32>() <= m { "Yes" } else { "No" });
}
