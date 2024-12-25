use proconio::{input as I, marker::Usize1 as U};
use petgraph::unionfind::UnionFind;

fn main() {
    I! {
        n: usize,
        uv: [(U, U)],
    }
    let mut g = UnionFind::new(n);
    for &(u, v) in &uv {
        g.union(u, v);
    }
    let mut h = std::collections::HashSet::new();
    for i in 0..n {
        h.insert(g.find(i));
    }
    println!("{}", h.len());
}
