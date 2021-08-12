const M: u64 = 1000_000_007;

fn main() {
    proconio::input! {
        n: usize,
        a: [[u64; 6]; n]
    }
    println!("{}", a.iter().fold(1, |s, a| s * a.iter().fold(0, |s, a| (s + a) % M) % M));
}
