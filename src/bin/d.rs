use proconio::{input as I, marker::Usize1 as U};
use petgraph::unionfind::UnionFind as T;

fn main() {
    I! {
        n: usize,
        e: [(U, U)]
    }
    let mut c = vec![0; n];
    let mut t = T::new(n);
    for (i, j) in e {
        c[i] += 1;
        c[j] += 1;
        t.union(i, j);
    }
    let mut d = vec![0; n];
    for i in 0..n {
        d[t.find(i)] += 1;
    }
    let mut r = 0;
    for i in 0..n {
        r += d[t.find(i)] - c[i] - 1;
    }
    println!("{}", r / 2);
}
