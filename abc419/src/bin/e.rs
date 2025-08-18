fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        l: usize,
        a: [usize; n]
    }
    const M: usize = 1000000;
    let mut dp = vec![M; m];
    dp[0] = 0;
    for i in 0..l {
        let mut t = vec![M; m];
        for j in 0..m {
            let c = ((i..n).step_by(l)).fold(0, |c, k| c + (m + j - a[k]) % m);
            for k in 0..m {
                t[(j + k) % m] = t[(j + k) % m].min(dp[k] + c);
            }
        }
        dp = t;
    }
    println!("{}", dp[0]);
}
