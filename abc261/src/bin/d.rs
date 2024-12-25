use proconio::{input as I};
fn main() {
    I! {
        n: usize,
        m: usize,
        x: [u64; n],
    }

    let mut d = vec![0; n + 1];
    for _ in 0..m {
        I! {c: usize, y: u64}
        d[c] = y;
    }

    let mut dp = vec![vec![0; n + 1]; n + 1];
    let mut t = 0;
    for i in 1..=n {
        dp[i][0] = t;
        t = 0;
        for j in 1..=i {
            dp[i][j] = dp[i - 1][j - 1] + x[i - 1] + d[j];
            t = t.max(dp[i][j]);
        }
    }

    println!("{}", t);
}
