use std::u64;

use proconio::{input as I, marker::Usize1 as U};
use itertools::Itertools;

fn main() {
    I! {n: usize}

    let mut g = vec![vec![false; n]; n];
    I! [m: usize];
    for _ in 0..m {
        I! {
            u: U,
            v: U
        }
        g[u][v] = true;
        g[v][u] = true;
    }

    let mut h = vec![vec![false; n]; n];
    I! [m: usize];
    for _ in 0..m {
        I! {
            u: U,
            v: U
        }
        h[u][v] = true;
        h[v][u] = true;
    }

    let mut c = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        I! [a: [u64; n - 1 - i]];
        for j in 0..n - 1 - i {
            c[i][i + 1 + j] = a[j];
        }
    }

    let mut m = u64::MAX;
    for p in (0..n).permutations(n) {
        let mut t = 0;
        for i in 0..n {
            for j in i + 1..n {
                let (u, v) = if p[i] < p[j] {(p[i], p[j])} else {(p[j], p[i])};
                if g[i][j] != h[u][v] {
                    t += c[u][v];
                }
            }
        }
        m = m.min(t);
    }

    println!("{}", m);
}
