const M: u32 = 1000_000_007;

fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        v: [[u32; k]; n],
    }

    let mut dp = vec![vec![0; k]; n];
    dp[0] = vec![1; k];
    for i in 1..n {
        let mut s = 0;
        let mut r = 0;
        for j in 0..k {
            while r < k && v[i - 1][r] <= v[i][j] {
                s += dp[i - 1][r];
                s %= M;
                r += 1;
            }
            dp[i][j] = s;
        }
    }
    println!("{}", dp[n - 1].iter().fold(0, |s, e| (s + e) % M));
}
