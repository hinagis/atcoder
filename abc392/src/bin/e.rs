use itertools::Itertools;
use proconio::{input as I, fastout as F, marker::Usize1 as U};
use petgraph::unionfind::UnionFind;

#[F]
fn main() {
    I! {
        n: usize,
        m: usize,
    }
    let mut t = UnionFind::new(n);
    let mut r = vec![vec![]; n];
    for i in 1..=m {
        I! {
            a: U,
            b: U
        }
        if t.equiv(a, b) {
            r[a].push((i, b));
        } else {
            t.union(a, b);
        }
    }
    let mut h = std::collections::HashSet::from(t.into_labeling());
    dbg!(&h);
    let mut a = vec![];
    for i in 0..n {
        h.remove(&t.find(i));
        if h.len() == 0 {
            break;
        }
        for &(j, b) in r[i].iter() {
            a.push((j, b + 1, t.find(b) + 1));
            t.union(i, b);
        }
    }
    println!("{}", a.len());
    for (i, b, a) in a {
        println!("{} {} {}", i, b, a);
    }
}
