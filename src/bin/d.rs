use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, u64); m]
    }

    let mut d = vec![vec![1 << 60; n]; n];
    for i in 0..n {
        d[i][i] = 0;
    }
    for &(a, b, c) in &abc {
        d[a][b] = c
    }

    let mut r = 0;
    for k in 0..n {
        let mut nxt = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                nxt[i][j] = d[i][j].min(d[i][k] + d[k][j]);
                if nxt[i][j] < 1 << 59 {
                    r += nxt[i][j]
                }
            }
        }
        d = nxt
    }
    println!("{}", r);
}
