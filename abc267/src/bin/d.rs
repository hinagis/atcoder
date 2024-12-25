fn main() {
    proconio::input!{
        n: usize,
        m: usize,
        a: [i64; n],
    }

    let mut dp = vec![std::i64::MIN; m + 1];
    for i in 0..n {
        for j in (2..=m.min(i + 1)).rev() {
            dp[j] = dp[j].max(a[i] * j as i64 + dp[j - 1]);
        }
        dp[1] = dp[1].max(a[i]);
    }

    println!("{}", dp[m]);
}
