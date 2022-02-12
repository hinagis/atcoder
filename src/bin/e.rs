use proconio::{input as I, marker::Usize1 as U1};

fn main() {
    I! {
        n: usize,
        m: usize,
        h: [i64; n]
    }

    let mut g = vec![vec![]; n];
    for _ in 0..m {
        I! {u: U1, v: U1}
        g[u].push((v, (h[v] - h[u]).max(0)));
        g[v].push((u, (h[u] - h[v]).max(0)));
    }

    let mut q = std::collections::BinaryHeap::new();
    q.push((0, 0));
    let mut p = vec![std::i64::MAX; n];
    p[0] = 0;

    while let Some((d, u)) = q.pop() {
        if p[u] >= -d {
            for &(v, c) in &g[u] {
                if p[v] > p[u] + c {
                    p[v] = p[u] + c;
                    q.push((-p[v], v));
                }
            }
        }
    }

    println!("{}", p
        .iter()
        .enumerate()
        .map(|(i, &x)| -x - h[i] + h[0])
        .max()
        .unwrap()
    );
}
