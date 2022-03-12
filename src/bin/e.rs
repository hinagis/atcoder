use proconio::{input as I, marker::Usize1 as U1};

fn main() {
    I! {
        n : usize,
        m : usize,
        e : [(U1, U1, u32); m]
    }

    let mut g = vec![vec![1_000_000_000; n]; n];
    for &(a, b, c) in &e {
        g[a][b] = c;
        g[b][a] = c;
    }

    let mut d = vec![vec![false; n]; n];
    for i in 0..n { for j in 0..n { for k in 0..n {
        if g[j][k] >= g[j][i] + g[i][k] {
            g[j][k] = g[j][i] + g[i][k];
            d[j][k] = true;
        }
    }}}

    println!("{}", e.iter().filter(|&&(a, b, _)| d[a][b]).count());
}