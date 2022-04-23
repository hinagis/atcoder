fn main() {
    proconio::input! {
        n: usize,
        p: u64
    }

    let mut dp = vec![vec![[0; 2]; n * 3 + 3]; n + 2];
    dp[1][0][1] = 1;
    dp[1][1][0] = 1;
    for i in 1..=n {
        for d in 0..=n * 3 {
            dp[i + 1][d + 2][0] += dp[i][d][1] * 2;
            dp[i + 1][d + 2][0] %= p;
            dp[i + 1][d + 1][1] += dp[i][d][1] * 3;
            dp[i + 1][d + 1][1] %= p;
            dp[i + 1][d + 1][0] += dp[i][d][0];
            dp[i + 1][d + 1][0] %= p;
            dp[i + 1][d][1] += dp[i][d][1] + dp[i][d][0];
            dp[i + 1][d][1] %= p;
        }
    }
    println!("{}", dp[n][1..n].iter().map(|v| v[1].to_string()).collect::<Vec<_>>().join(" "));
}
