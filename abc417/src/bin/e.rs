use itertools::Itertools;
use std::collections::VecDeque as Q;
use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {t: u32}
    for _ in 0..t {
        I! {
            n: usize,
            m: usize,
            x: U,
            y: U
        }
        let mut g = vec![vec![]; n];
        for _ in 0..m {
            I! {u: U, v: U}
            g[u].push(v);
            g[v].push(u);
        }
        for i in 0..n {
            g[i].sort();
        }
        let mut f = vec![false; n];
        let mut p = Q::new();
        dfs(&g, &mut f, &mut p, y, x);
    }
}

fn dfs(g: &Vec<Vec<usize>>, f: &mut Vec<bool>, p: &mut Q<usize>, y: usize, i: usize) -> bool {
    if f[i] {return false}
    f[i] = true;
    p.push_back(i + 1);
    for &j in &g[i] {
        if f[j] {continue}
        if j == y {
            p.push_back(y + 1);
            println!("{}", p.iter().join(" "));
            return true;
        }
        if dfs(g, f, p, y, j) {return true}
    }
    p.pop_back();
    false
}
