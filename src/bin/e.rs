use proconio::{input as I, fastout as F, marker::Usize1 as U};

fn main() {
    I! {t: usize}
    for _ in 0..t {
        solve();
    }
}

#[F]
fn solve() {
    I! {
        n: usize,
        m: usize,
        c: [u8; n],
    }

    let mut t = vec![vec![]; n];
    for _ in 0..m {
        I!{
            u: U,
            v: U
        }
        t[u].push(v);
        t[v].push(u);
    }
    dbg!(&t);
    let mut f = vec![vec![false; n]; n];
    let mut q = std::collections::VecDeque::new();
    q.push_back((0, n - 1, 0));
    while let Some((u, v, d)) = q.pop_front() {
        if u == n - 1 && v == 0 {
            println!("{}", d);
            return;
        }
        for &i in &t[u] {
            for &j in &t[v] {
                if f[i][j] || c[i] == c[j] {
                    continue;
                }
                f[i][j] = true;
                q.push_back((i, j, d + 1));
            }
        }
    }
    println!("-1");
}
