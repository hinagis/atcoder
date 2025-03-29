use itertools::Itertools;
use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        m: usize,
        e: [(U, U); m]
    }

    let mut g = petgraph::unionfind::UnionFind::new(n);
    for (u, v) in e {
        g.union(u, v);
    }
    println!("{}", m + g.into_labeling().iter().unique().count() - n);
}
