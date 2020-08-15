use proconio::{input, marker::Usize1};
use num::bigint::BigUint;

fn main() {
    input! {
        n: usize,
        h: [Usize1; n],
        a: [u32; n],
    }

    let mut dp = vec![vec![BigUint::new(vec![0]); n]; n];
    for j in h[0]..n {
        dp[0][j] += a[0];
    }
    for i in 1..n {
        for j in 0..h[i] {
            dp[i][j] = dp[i - 1][j].clone();
        }
        dp[i][h[i]] = dp[i - 1][h[i]].clone() + a[i];
        for j in (h[i] + 1)..n {
            dp[i][j] = if dp[i - 1][j] > dp[i][h[i]] {
                dp[i - 1][j].clone()
            } else {
                dp[i][h[i]].clone()
            }
        }
    }

    let mut m = &BigUint::new(vec![0]);
    for j in 0..n {
        if dp[n - 1][j] > *m {
            m = &dp[n - 1][j]
        }
    }
    println!("{}", *m);
}
