const M: i32 = 1000;

fn main() {
    proconio::input! {
        n: usize,
        x: usize,
        y: usize,
        ab: [(usize, usize); n]
    }
    let mut dp = vec![vec![vec![M; y + 1]; x + 1]; n + 1];
    dp[0][0][0] = 0;
    for i in 1..=n {
        for j in 0..=x {
            for k in 0..=y {
                let x = x.min(j + ab[i - 1].0);
                let y = y.min(k + ab[i - 1].1);
                dp[i][x][y] = dp[i][x][y].min(dp[i - 1][j][k] + 1);
                dp[i][j][k] = dp[i][j][k].min(dp[i - 1][j][k]);
            }
        }
    }

    println!("{}", if dp[n][x][y] < M {dp[n][x][y]} else {-1});
}
