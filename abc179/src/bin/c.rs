fn main() {
    proconio::input! {
        n: usize,
    }
    let mut dp = vec![0; n];
    for i in 1..n {
        for j in 1..n {
            if i * j >= n {
                break;
            } else {
                dp[i * j] += 1;
            }
        }
    }
    let mut r = 0;
    for c in 1..n {
        r += dp[n - c];
    }

    println!("{}", r);
}
