use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        t: C
    }
    let mut dp = vec![[0u64; 2]; n + 1];
    for i in 0..n {
        if t[i] == '0' {
            dp[i + 1][0] = dp[i][1];
            dp[i + 1][1] = dp[i][0] + 1;
        } else {
            dp[i + 1][0] = dp[i][0] + 1;
            dp[i + 1][1] = dp[i][1];
        }
    }
    println!("{}", (1..=n).fold(0, |s, i| s + dp[i][0]));
}
