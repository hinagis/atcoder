fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        c: u64,
        mut a: [[u64; w]; h]
    }

    let mut r = u64::max_value();
    for _ in 0..2 {
        let mut dp = vec![vec![0; w]; h];
        dp[0][0] = a[0][0];
        for i in 1..h {
            dp[i][0] = a[i][0].min(dp[i - 1][0] + c);
            r = r.min(a[i][0] + dp[i - 1][0] + c);
        }
        for j in 1..w {
            dp[0][j] = a[0][j].min(dp[0][j - 1] + c);
            r = r.min(a[0][j] + dp[0][j - 1] + c);
        }
        for i in 1..h {
            for j in 1..w {
                dp[i][j] = a[i][j].min(dp[i][j - 1].min(dp[i - 1][j]) + c);
                r = r.min(a[i][j] + dp[i][j - 1].min(dp[i - 1][j]) + c);
            }
        }
    
        for i in 0..h {
            a[i].reverse();
        }
    }

    println!("{}", r);
}
