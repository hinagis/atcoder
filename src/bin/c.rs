use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        x: U,
        y: U,
        uv: [(U, U); n - 1]
    }
    let mut t = vec![vec![]; n];
    for (u, v) in uv {
        t[u].push(v);
        t[v].push(u);
    }
    let mut f = vec![n; n];
    f[y] = y;
    let mut q = std::collections::VecDeque::new();
    q.push_back(y);
    while let Some(p) = q.pop_front() {
        for &i in &t[p] {
            if f[i] >= n {
                f[i] = p;
                q.push_back(i);
            }
        }
    }

    let mut i = x;
    while i != y {
        println!("{}", i + 1);
        i = f[i];
    }
    println!("{}", y + 1);
}
