use itertools::iproduct;

fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        a: [[i64; w]; h],
        p: [i64; h + w - 1]
    }
    let mut dp = vec![vec![i64::MAX; w]; h];
    dp[h - 1][w - 1] = 0.max(p[h + w - 2] - a[h - 1][w - 1]);
    for i in (0..h - 1).rev() {
        dp[i][w - 1] = 0.max(dp[i + 1][w - 1] - a[i][w - 1] + p[i + w - 1]);
    }
    for j in (0..w - 1).rev() {
        dp[h - 1][j] = 0.max(dp[h - 1][j + 1] - a[h - 1][j] + p[h - 1 + j]);
    }
    for (i, j) in iproduct!((0..h - 1).rev(), (0..w - 1).rev()) {
        dp[i][j] = 0.max(dp[i + 1][j].min(dp[i][j + 1]) - a[i][j] + p[i + j]);
    }
    println!("{}", dp[0][0]);
}
