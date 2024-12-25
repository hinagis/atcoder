fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        a: [usize; k],
    }

    let mut dp = vec![0; n + 1];
    for i in 0..=n {
        for j in 0..k {
            if a[j] > i {
                break;
            }
            dp[i] = dp[i].max(i - dp[i - a[j]]);
        }
    }

    println!("{}", dp[n]);
}
