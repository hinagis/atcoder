fn main() {
    proconio::input!{
        n: usize,
        k: usize,
        d: usize,
        a: [usize; n],
    };
    let mut dp = vec![vec![-1i64; d]; k + 1];
    dp[0][0] = 0;
    for a in a {
        for i in (0..k).rev() {
            for j in 0..d {
                let v = dp[i][j];
                if v < 0 {continue}
                let p = &mut dp[i + 1][(j + a) % d];
                *p = (*p).max(v + a as i64);
            }
        }
    }
    println!("{}", dp[k][0]);
}