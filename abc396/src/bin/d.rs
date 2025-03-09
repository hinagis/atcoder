use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        m: usize,
    }
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        I! {
            u: U,
            v: U,
            w: u64,
        }
        g[u].push((v, w));
        g[v].push((u, w));
    }
    let mut r = u64::MAX;
    let mut q = std::collections::VecDeque::new();
    let mut f = vec![false; n];
    f[0] = true;
    f[n - 1] = true;
    q.push_back((0, 0, f));
    while let Some((u, b, f)) = q.pop_front() {
        for &(v, w) in g[u].iter() {
            if v == n - 1 {
                r = r.min(b ^ w);
            }
            if f[v] {continue}
            let mut f = f.clone();
            f[v] = true;
            q.push_back((v, b ^ w, f));
        }
    }
    println!("{}", r);
}
