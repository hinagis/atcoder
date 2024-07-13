use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        m: usize,
        a: [u64; n],
        b: [(U, U, u64); m]
    }
    let mut t = vec![vec![]; n];
    for &(u, v, b) in &b {
        t[u].push((v, b));
        t[v].push((u, b));
    }

    let mut c = vec![u64::MAX; n];
    let mut q = std::collections::VecDeque::new();
    c[0] = a[0];
    for &(v, b) in &t[0] {
        c[v] = a[0] + b + a[v];
        q.push_back((v, c[v]));
    }
    while let Some((u, p)) = q.pop_front() {
        for &(v, b) in &t[u] {
            let d = p + b + a[v];
            if d < c[v] {
                c[v] = d;
                q.push_back((v, d));
            }
        }
    }
    println!("{}", c.iter().skip(1).map(|c| c.to_string()).collect::<Vec<_>>().join(" "));
}
