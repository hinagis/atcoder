const M: usize = 998244353;

fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        lr: [(usize, usize); k],
    }

    let mut dp = vec![0; n];
    dp[0] = 1;
    dp[1] = M - 1;
    for i in 0..(n - 1) {
        if dp[i] > 0 {
            for &(l, r) in &lr {
                let (l, r) = (i + l, i + r + 1);
                if l < n { dp[l] = (dp[l] + dp[i]) % M }
                if r < n { dp[r] = (M + dp[r] - dp[i]) % M }
            }
            dp[i + 1] = (dp[i + 1] + dp[i]) % M;
        }
    }
    println!("{}", dp[n - 1]);
}
