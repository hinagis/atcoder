use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {t: usize}
    for _ in 0..t {
        I! {
            n: usize,
            s: C
        }
        let mut dp = vec![[1000000; 3]; n];
        dp[0][0] = if s[0] == '0' {0} else {1};
        dp[0][1] = if s[0] == '1' {0} else {1};

        for i in 1..n {
            dp[i][0] = dp[i - 1][0] + if s[i] == '0' {0} else {1};
            dp[i][1] = dp[i - 1][0] + if s[i] == '1' {0} else {1};
            dp[i][1] = dp[i][1].min(dp[i - 1][1] + if s[i] == '1' {0} else {1});
            dp[i][2] = dp[i][2].min(dp[i - 1][1] + if s[i] == '0' {0} else {1});
            dp[i][2] = dp[i][2].min(dp[i - 1][2] + if s[i] == '0' {0} else {1});
        }
        println!("{}", dp[n - 1].iter().min().unwrap());
    }
}
