const M: i64 = 998244353;

fn main() {
    proconio::input! {
        n: usize, m: usize, k: usize,
        uv: [(usize, usize); m]
    }
    let mut e = vec![vec![true; n + 1]; n + 1];
    for i in 1..=n {
        e[i][i] = false;
    }
    for &(u, v) in &uv {
        e[u][v] = false;
        e[v][u] = false;
    }

    let mut dp = vec![vec![0; n + 1]; k + 1];
    dp[0][1] = 1;
    let mut q = std::collections::HashSet::new();
    q.insert(1);
    for k in 0..k {
        let mut nq = std::collections::HashSet::new();
        for &i in &q {
            for j in 1..=n {
                if e[i][j] {
                    dp[k + 1][j] = (dp[k + 1][j] + dp[k][i]) % M;
                    nq.insert(j);
                }
            }
        }
        q = nq;
    }
    println!("{}", dp[k][1]);
}
