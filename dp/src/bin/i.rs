//use proconio::{input, fastout};
use proconio::input;

//#[fastout]
fn main() {
    input! {
        n: usize,
        p: [f64; n],
    }
    let mut dp = vec![0.0; n + 1];
    dp[0] = 1.0;
    for i in 0..n {
        for j in (1..=(i + 1)).rev() {
            dp[j] = dp[j] * (1.0 - p[i]) + dp[j - 1] * p[i];
        }
        dp[0] = dp[0] * (1.0 - p[i]);
    }

    let mut r = 0.0;
    for i in (n / 2 + 1)..=n {
        r += dp[i];
    }
    println!("{}", r);
}
