const M: i64 = 998244353;

fn main() {
    proconio::input! {
        n: usize, m: usize, k: usize,
        uv: [(usize, usize); m]
    }
    let mut e = vec![vec![]; n + 1];
    for &(u, v) in &uv {
        e[u].push(v);
        e[v].push(u);
    }

    let mut dp = vec![vec![0; n + 1]; k + 1];
    dp[0][1] = 1;
    for k in 0..k {
        let s = dp[k].iter().fold(0, |s, c| s + c);
        for i in 1..=n {
            dp[k + 1][i] = s - dp[k][i];
            for &j in &e[i] {
                dp[k + 1][i] -= dp[k][j];
            }
            dp[k + 1][i] %= M;
        }
    }
    println!("{}", dp[k][1]);
}
