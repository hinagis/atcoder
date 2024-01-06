use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        m: usize,
        a: [u32; n],
        uv: [(U, U); m]
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut f = vec![true; n];
    let mut s = 0;
    let mut q = std::collections::VecDeque::new();
    q.push_back((0, 1));
    while let Some((u, c)) = q.pop_front() {
        if u == n - 1 {
            s = s.max(c);
            continue
        }

        if f[u] {
            f[u] = false;
            for &v in &g[u] {
                if f[v] && a[u] <= a[v] && a[v] <= a[n - 1] {
                    f[u] = true;
                    q.push_back((v, c + 1))
                }
            }
        }
    }
    println!("{}", s);
}
