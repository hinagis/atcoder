use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {n: usize}
    let mut g = vec![vec![]; n];
    for _ in 1..n {
        I! {u: U, v: U}
        g[u].push(v);
        g[v].push(u);
    }

    let l = g.iter().map(|u| u.len()).collect::<Vec<_>>();
    for i in 0..n {
        g[i].sort_by(|&a, &b| l[b].cmp(&l[a]));
    }
    let mut m = n;
    for i in 0..n {
        for j in 0..l[i] {
            m = m.min(n - 1 - (j + 1) * (l[g[i][j]]));
        }
    }
    println!("{}", m);
}
