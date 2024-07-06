fn main() {
    proconio::input! {
        n: usize,
        s: String,
        t: String
    }
    let mut dp = vec![vec![u64::MAX; 2usize.pow(n as u32)]; n + 1];
    let s = s.chars().fold(0, |s, c| (s << 1) | if c == 'B' {1} else {0});
    let t = t.chars().fold(0, |s, c| (s << 1) | if c == 'B' {1} else {0});
    dp[n][s] = 0;
    let mut f = true;
    fn calc(i: usize, j: usize, k: usize) -> Option<(usize, usize)> {
        if k != i && k + 1 != i && k != i + 1 {
            Some((i, j))
        } else {
            None
        }
    }
    while f {
        f = false;
        for i in 0..=n {
            for j in 0..2usize.pow(n as u32) {
                if dp[i][j] < u64::MAX {
                    for k in 0..=n {
                        if let Some((u, v)) = calc(i, j, k) {
                            if dp[i][j] + 1 < dp[u][v] {
                                f = true;
                                dp[u][v] = dp[i][j] + 1;
                            }
                        }
                    }
                }
            }
        }
    }
    let m = dp.iter().fold(u64::MAX, |s, v| s.min(v[t]));
    println!("{}", if m < u64::MAX {m as i64} else {-1});
}
