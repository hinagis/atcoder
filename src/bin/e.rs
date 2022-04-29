fn main() {
    proconio::input! {
        n: usize,
        p: i64,
    }

    let mut dp = vec![vec![0; n + 2]; n + 5];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..n {
            let v = dp[i][j] * if i != 0 {dp[i][j + 1] = (dp[i][j + 1] + dp[i][j]) % p; 25} else {26} % p;
            let mut l = 1;
            let mut k = 1;
            while l <= n - j {
                let r = (l * 10).min(n - j + 1);
                let di = k + 1;
                dp[i + di][j + l] = (dp[i + di][j + l] + v) % p;
                dp[i + di][j + r] = (dp[i + di][j + r] - v) % p;
                l *= 10;
                k += 1;
            }
        }
    }
    println!("{}", dp[..n].iter().fold(0, |s, v| (s + v[n] + p) % p));
}
