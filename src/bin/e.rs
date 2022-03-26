use proconio::{input as I, marker::Usize1 as U1};

const M: u64 = 998244353;

fn main() {
    I! {
        n: usize,
        m: usize,
        k: usize,
        s: U1,
        t: U1,
        x: U1,
        uv: [(U1, U1); m]
    }
    let mut e = vec![vec![]; n];
    for &(u, v) in &uv {
        e[u].push(v);
        e[v].push(u);
    }
    let mut dp = vec![vec![vec![0; 2]; n]; k + 1];
    dp[0][s][0] = 1;
    for i in 0..k {
        for j in 0..n {
            for &v in &e[j] {
                dp[i + 1][v][0] += dp[i][j][if v == x {1} else {0}];
                dp[i + 1][v][0] %= M;
                dp[i + 1][v][1] += dp[i][j][if v == x {0} else {1}];
                dp[i + 1][v][1] %= M;
            }
        }
    }

    println!("{}", dp[k][t][0]);
}
