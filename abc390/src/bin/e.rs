use std::i64;

use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        x: usize
    }
    let mut dp = vec![vec![i64::MIN; x + 1]; 3];
    for v in 0..3 {
        dp[v][0] = 0;
    }
    for _ in 0..n {
        I! {
            v: U,
            a: i64,
            c: usize
        }
        for j in (c..=x).rev() {
            dp[v][j] = dp[v][j].max(dp[v][j - c] + a);
        }
    }
    for v in 0..3 {
        for j in 0..x {
            dp[v][j + 1] = dp[v][j + 1].max(dp[v][j]);
        }
    }
    let mut p = [0; 3];
    for _ in 0..x {
        let a = [dp[0][p[0]], dp[1][p[1]], dp[2][p[2]]];
        p[if a[0] <= a[1] && a[0] <= a[2] {
            0
        } else if a[1] <= a[0] && a[1] <= a[2] {
            1
        } else {
            2
        }] += 1;
    }
    println!("{}", dp[0][p[0]].min(dp[1][p[1]]).min(dp[2][p[2]]));
}
