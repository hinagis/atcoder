fn main() {
    proconio::input! {
        l: u64,
    }
    println!("{}", binom_knuth(l - 1, 11));
}

pub fn binom_knuth(n: u64, k: u64) -> u64 {
    (0..n + 1)
        .rev()
        .zip(1..k + 1)
        .fold(1, |mut r, (n, d)| { r *= n; r /= d; r })
}
