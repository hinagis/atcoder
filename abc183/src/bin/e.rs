use proconio::{input, marker::Chars};

const M: u64 = 1000_000_007;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut x = vec![vec![0; w]; h];
    let mut y = vec![vec![0; w]; h];
    let mut z = vec![vec![0; w]; h];
    let mut dp = vec![vec![0; w]; h];
    dp[0][0] = 1;
    for j in 1..w {
        if s[0][j] == '.' {
            x[0][j] = (x[0][j - 1] + dp[0][j - 1]) % M;
            dp[0][j] = x[0][j];
        }
    }
    for i in 1..h {
        if s[i][0] == '.' {
            y[i][0] = (y[i - 1][0] + dp[i - 1][0]) % M;
            dp[i][0] = y[i][0];
        }
        for j in 1..w {
            if s[i][j] == '.' {
                x[i][j] = (x[i][j - 1] + dp[i][j - 1]) % M;
                y[i][j] = (y[i - 1][j] + dp[i - 1][j]) % M;
                z[i][j] = (z[i - 1][j - 1] + dp[i - 1][j - 1]) % M;
                dp[i][j] = (x[i][j] + y[i][j] + z[i][j]) % M;
            }
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}
