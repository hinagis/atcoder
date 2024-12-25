fn main() {
    proconio::input! {
        n: usize,
        t: [usize; n]
    }
    let a = t.iter().fold(0, |s, &t| s + t);
    let mut dp = vec![vec![false; a + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        for j in 0..a + 1 {
            dp[i + 1][j] |= dp[i][j];
            if j >= t[i] {
                dp[i + 1][j] |= dp[i][j - t[i]];
            }
        }
    }

    let mut j = a - a / 2;
    while ! dp[n][j] {
        j += 1;
    }
    println!("{}", j);
}
