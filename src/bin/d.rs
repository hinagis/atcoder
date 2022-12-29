fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        d: usize,
        a: [usize; n],
    }
    let mut dp = vec![vec![vec![-1i64; d]; k + 1]; n + 1];
    dp[0][0][0] = 0;
    for i in 0..n {
        for j in 0..=k {
            for m in 0..d {
                let v = dp[i][j][m];
                if v == -1 {
                    continue;
                }
                let p = &mut dp[i + 1][j][m];
                *p = (*p).max(v);
                if j != k {
                    let p = &mut dp[i + 1][j + 1][(m + a[i]) % d];
                    *p = (*p).max(v + a[i] as i64);
                }
            }
        }
    }
    println!("{}", dp[n][k][0]);
}
