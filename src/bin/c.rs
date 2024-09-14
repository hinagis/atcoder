use proconio::{input as I, marker::Usize1 as U};
use itertools::Itertools;

fn main() {
    I! {n: usize}

    let mut g = vec![vec![]; n];
    I! [m: usize];
    for _ in 0..m {
        I! {
            u: U,
            v: U
        }
        g[u].push(v);
        g[v].push(u);
    }

    let mut h = vec![vec![]; n];
    I! [m: usize];
    for _ in 0..m {
        I! {
            u: U,
            v: U
        }
        h[u].push(v);
        h[v].push(u);
    }

    let mut c = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        I! [a: [U; n - 1 - i]];
        for j in 0..i {
            c[i][i + 1 + j] = a[j];
        }
    }

    
    for p in (0..n).permutations(n) {

    }

    println!("{}", n);
}
