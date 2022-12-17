use proconio::{input as I, marker::Usize1 as U};
use petgraph::unionfind::UnionFind;

fn main() {
    I! {
        n: usize,
        m: usize,
        uv: [(U, U); m]
    }

    let mut t = vec![vec![]; n];
    for &(u, v) in &uv {
        t[u].push(v);
        t[v].push(u);
    }

    let mut c = vec![true; n];
    let mut f = vec![true; n];
    for i in 0..n {
        if f[i] {
            let mut q = std::collections::VecDeque::new();
            q.push_back((i, false));
            while let Some((e, x)) = q.pop_front() {
                if f[e] {
                    f[e] = false;
                    c[e] = x;
                    for &j in &t[e] {
                        if f[j] {
                            q.push_back((j, !x));
                        } else {
                            if c[j] == x {
                                println!("0");
                                return;
                            }
                        }
                    }
                } else {
                    if c[e] != x {
                        println!("0");
                        return;
                    }
                }
            }
        }
    }

    let mut y = UnionFind::new(n);
    for &(u, v) in &uv {
        y.union(u, v);
    }

    let mut s = vec![[0, 0]; n];
    for i in 0..n {
        s[y.find(i)][if c[i] {0} else {1}] += 1;
    }

    let mut r = 0;
    for i in 0..n {
        r += n - t[i].len() - s[y.find(i)][if c[i] {0} else {1}];
    }
    println!("{}", r / 2);
}
