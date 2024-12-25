const M: usize = 1000_000_007;

fn main() {
    proconio::input! {
        s: usize,
    }

    if s >= 3 {
        if s >= 6 {
            let mut dp = vec![0; s + 1];
            dp[3] = 1;
            dp[4] = 1;
            dp[5] = 1;
            for i in 6..=s {
                dp[i] = (dp[i - 1] + dp[i - 3]) % M;
            }
            println!("{}", dp[s]);
        } else {
            println!("1");
        }
    } else {
        println!("0");
    }
}
