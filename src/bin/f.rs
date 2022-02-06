use proconio::{input as I, marker::Usize1 as U1};
const M: u64 = 998244353;

fn main() {
    I! {n: usize, k: usize}

    let a = (|| {
        let mut a = vec![0; n];
        I! {p: [U1; n]}
        for &p in &p {
            I!{q: U1}
            a[p] = q;
        }
        a
    })();

    let mut dp = vec![vec![0; n + 1]; k + 1];
    dp[0][n] = 1;
    for i in 0..n {
        let mut t = vec![vec![0; n + 1]; k + 1];
        for u in 0..=k {
            for v in 0..=n {
                if u < k && a[i] < v {
                    t[u + 1][v] += dp[u][v];
                    t[u + 1][v] %= M;
                }
                t[u][v.min(a[i])] += dp[u][v];
                t[u][v.min(a[i])] %= M;
            }
        }
        std::mem::swap(&mut dp, &mut t);
    }

    println!("{}", dp[k].iter().fold(0, |s, v| (s + v) % M));
}
