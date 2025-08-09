use itertools::iproduct;
use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    const M: u64 = std::u64::MAX / 3;
    I! {
        n: usize,
        e: [(U, U, u64)],
        k: usize,
        t: u64,
        d: [U; k],
        q: usize,
    }
    let mut g = vec![vec![M; n]; n];
    for i in 0..n {
        g[i][i] = 0;
    }
    for (a, b, c) in e {
        g[a][b] = g[a][b].min(c);
        g[b][a] = g[b][a].min(c);
    }
    let mut f = vec![false; n];
    for i in 0..d.len() {
        f[d[i]] = true;
        for j in 0..d.len() {
            g[d[i]][d[j]] = g[d[i]][d[j]].min(t);
        }
    }
    for (k, i, j) in iproduct!(0..n, 0..n, 0..n) {
        g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
    }
    for _ in 0..q {
        I!(q: u8);
        if q == 1 {
            I! {
                a: U,
                b: U,
                c: u64,
            }
            for (i, j) in iproduct!(0..n, 0..n) {
                g[i][j] = g[i][j].min(g[i][a] + c + g[b][j]).min(g[i][b] + c + g[a][j]);
            }
        } else if q == 2 {
            I!(x: U);
            if !f[x] {
                f[x] = true;
                for i in 0..n {
                    if !f[i] {
                        continue;
                    }
                    for j in 0..n {
                        let d = g[j][i] + t;
                        g[j][x] = g[j][x].min(d);
                        g[x][j] = g[x][j].min(d);
                    }
                }
                for (i, j) in iproduct!(0..n, 0..n) {
                    g[i][j] = g[i][j].min(g[i][x] + g[x][j]);
                }
            }
        } else {
            println!("{}", g.iter().flatten().filter(|&&g| g < M).sum::<u64>());
        }
    }
}
