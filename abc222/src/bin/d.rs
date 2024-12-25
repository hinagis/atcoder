const M: u64 = 998244353;

fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut dp = vec![vec![0; 3002]; n + 1];
    for j in 1..3002 {
        dp[0][j] = 1;
    }

    for i in 0..n {
        for j in a[i] + 1..=b[i] + 1 {
            dp[i + 1][j] = (dp[i][j] + dp[i + 1][j - 1]) % M;
        }
        for j in b[i] + 2..3002 {
            dp[i + 1][j] = dp[i + 1][j - 1];
        }
    }

    println!("{}", dp[n][b[n - 1] + 1]);
}
