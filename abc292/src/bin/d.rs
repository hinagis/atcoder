use proconio::{input as I, marker::Usize1 as U};
use petgraph::unionfind::UnionFind as T;

fn main() {
    I! {
        n: usize,
        e: [(U, U)]
    }
    let mut t = T::new(n);
    for &(u, v) in &e {
        t.union(u, v);
    }
    let mut d = vec![0; n];
    let mut c = vec![(std::collections::HashSet::new(), 0); n];
    for &(u, v) in &e {
        let i = t.find(u);
        c[i].0.insert(u);
        c[i].0.insert(v);
        c[i].1 += 1;
        d[u] += 1;
        d[v] += 1;
    }
    for i in 0..n {
        if c[i].0.len() != c[i].1 || d[i] == 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
