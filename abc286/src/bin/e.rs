use proconio::{input as I, fastout as F, marker::{Usize1 as U, Chars as C}};

#[F]
fn main() {
    I! {
        n: usize,
        a: [u64; n],
    }

    let mut d = vec![vec![(n, 0); n]; n];
    for i in 0..n {
        d[i][i].0 = 0;
    }
    for i in 0..n {
        I! {s: C}
        for j in 0..n {
            if s[j] == 'Y' {
                d[i][j] = (1, a[j]);
            }
        }
    }
    
    for j in 0..n {
        for i in 0..n {
            for k in 0..n {
                if d[i][j].0 + d[j][k].0 < d[i][k].0 {
                    d[i][k] = (d[i][j].0 + d[j][k].0, d[i][j].1 + d[j][k].1);
                } else if (d[i][j].0 + d[j][k].0 == d[i][k].0) && (d[i][j].1 + d[j][k].1 > d[i][k].1) {
                    d[i][k].1 = d[i][j].1 + d[j][k].1;
                }
            }
        }
    }

    I! {uv: [(U, U)]}
    for &(u, v) in &uv {
        if d[u][v].0 == n {
            println!("Impossible");
        } else {
            println!("{} {}", d[u][v].0, d[u][v].1 + a[u]);
        }
    }
}
