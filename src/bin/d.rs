fn main() {
    const T: usize = 100_000;
    proconio::input! {
        n: usize,
        txa: [(usize, usize, u64); n]
    }
    let mut dp = vec![[0; 5]; T + 1];
    let mut j = 0;
    for t in 0..T {
        dp[t + 1][0] = dp[t][0].max(dp[t][1]);
        for i in 1..4 {
            dp[t + 1][i] = dp[t][i].max(dp[t][i - 1]).max(dp[t][i + 1]);
        }
        dp[t + 1][4] = dp[t][4].max(dp[t][3]);

        if j < n && t + 1 == txa[j].0 {
            let (_, x, a) = txa[j];
            if t + 1 >= x {
                dp[t + 1][x] += a;
            }
            j += 1;
        }
    }

    let mut m = 0;
    for i in 0..5 {
        m = m.max(dp[T][i]);
    }
    println!("{}", m);
}
