use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }
    let mut dp = vec![vec![vec![vec![10000; n]; n]; n]; n];
    println!("{}", calc(&s, &mut dp, 0, 0, n - 1, n - 1));
}

fn calc(s: &Vec<Vec<char>>, dp: &mut Vec<Vec<Vec<Vec<usize>>>>, d: usize, l: usize, t: usize, r: usize) -> usize {
    if l > r || d > t {
        0
    } else {
        let mut m = dp[d][l][t][r];
        if m >= 1000 {
            m = (t - d + 1).max(r - l + 1);
            for i in d..=t {
                if (l..=r).all(|j| s[i][j] == '.') {
                    let a = calc(s, dp, i + 1, l, t, r);
                    let b = if i > 0 {calc(s, dp, d, l, i - 1, r)} else {0};
                    m = m.min(a + b);
                }
            }
            for j in l..=r {
                if (d..=t).all(|i| s[i][j] == '.') {
                    let a = calc(s, dp, d, j + 1, t, r);
                    let b = if j > 0 {calc(s, dp, d, l, t, j - 1)} else {0};
                    m = m.min(a + b);
                }
            }
            dp[d][l][t][r] = m;
        }
        m
    }
}
