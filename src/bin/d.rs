use proconio::{input as I, marker::Usize1 as U};
use petgraph::unionfind::UnionFind as T;

fn main() {
    I! {
        n: usize,
        e: [(U, U)]
    }
    let mut t = T::new(n);
    for &(i, j) in &e {
        t.union(i, j);
    }
    let mut c = std::collections::HashMap::new();
    for i in 0..n {
        *c.entry(t.find(i)).or_insert(0) += 1;
    }
    let mut r = 0;
    for &v in c.values() {
        r += v * (v - 1) / 2;
    }
    println!("{}", r - e.len());
}
