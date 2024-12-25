const M: u64 = 1000_000_007;

fn main() {
    proconio::input! {k: usize}
    println!("{}",  if k % 9 == 0 {
        let mut dp = vec![0; k + 1];
        dp[0] = 1;
        for i in 1..=k {
            let k = i.min(9);
            for j in 1..=k {
                dp[i] = (dp[i] + dp[i - j]) % M
            }
        }
        dp[k]
    } else {
        0
    });
}
