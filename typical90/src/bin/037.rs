fn main() {
    proconio::input! {
        w: usize,
        n: usize,
        lrv: [(usize, usize, i64); n],
    }

    let mut dp = vec![vec![-1; w + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        let &(l, r, v) = &lrv[i - 1];
        for j in 0..=w {
            let p = dp[i - 1][j];
            if p < 0 {continue}

            dp[i][j] = dp[i][j].max(p);
            let v = p + v;
            if j + l <= w {
                dp[i][j + l] = dp[i][j + l].max(v);
                if j + r > w {
                    dp[i][w] = dp[i][w].max(v);
                }
            }
            if j + r <= w {
                dp[i][j + r] = dp[i][j + r].max(v);
            }
        }
    }
    println!("{}", dp[n][w]);
}
