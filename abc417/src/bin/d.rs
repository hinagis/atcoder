use proconio::{input as I, fastout as F};
use superslice::Ext;

#[F]
fn main() {
    I! {
        n: usize,
        v: [(usize, usize, usize); n],
        q: usize,
        x: [usize; q],
    }

    let m = v.iter().map(|&(p, a, _)| p + a).max().unwrap();

    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0..=m {
        dp[n][i] = i;
    }

    for i in (0..n).rev() {
        let (p, a, b) = v[i];
        for j in 0..=m {
            dp[i][j] = if j <= p {dp[i + 1][j + a]} else {dp[i + 1][j - j.min(b)]}
        }
    }

    let c = {
        let mut c = 0;
        v.iter().map(|&(_, _, b)| {c += b; c}).collect::<Vec<_>>()
    };

    for x in x {
        println!("{}", if x <= m {
                dp[0][x]
            } else {
                let i = c.lower_bound(&(x - m));
                if i < n {dp[i + 1][x - x.min(c[i])]} else {x - c[n - 1]}
        });
    }
}
