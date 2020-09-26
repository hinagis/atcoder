fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut dp = vec![0; n];
    dp[0] = 1;
    for i in 0..n {
        for j in (i + 1)..n {
            let (a, b) = if a[i] >= a[j] {(a[i], a[j])} else {(a[j], a[i])};
            if a - b <= k {
                dp[j] = dp[j].max(dp[i] + 1);
            }
        }
    }

    println!("{}", dp[n - 1]);
}
