fn main() {
    proconio::input! {
        n: usize,
        w: i64,
        stp: [(usize, usize, i64); n],
    }
    let mut dp = vec![0; 2 * 1000000];
    let mut e = 0;
    for &(s, t, p) in &stp {
        dp[s] += p;
        dp[t] -= p;
        e = e.max(t + 1);
    }
    let mut p = 0;
    for i in 0..e {
        p += dp[i];
        if p > w {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
