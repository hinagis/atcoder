use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        uv: [(U, U)]
    }
    let mut t = vec![vec![]; n];
    for &(u, v) in &uv {
        t[u].push(v);
        t[v].push(u);
    }
    let mut f = vec![true; n];
    let mut k = 0;
    bfs(&t, &mut f, &mut k, 0);
    println!("{}", k.min(1000_000));
}

fn bfs(t: &Vec<Vec<usize>>, f: &mut Vec<bool>, k: &mut u32, i: usize) {
    *k += 1;
    f[i] = false;
    for &e in &t[i] {
        if f[e] {
            bfs(t, f, k, e);
            if *k > 1000_000 {
                return;
            }
        }
    }
    f[i] = true;
}
