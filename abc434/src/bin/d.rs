use proconio::{input as I, fastout as F, marker::Usize1 as U};
use std::{collections::BTreeMap as M, iter::FromIterator};

#[F]
fn main() {
    I! {
        n: usize,
        udlr: [(U, usize, U, usize); n]
    }
    let mut t = vec![M::from_iter(vec![(0, 0), (2000, 2)]); 2000];
    for (u, d, l, r) in udlr {
        for i in u..d {
            let (_, &c) = t[i].range(..=l).last().unwrap();
            *t[i].entry(l).or_insert(c) += 1;
            let (_, &c) = t[i].range(..=r).last().unwrap();
            t[i].entry(r).or_insert(c);
            let mut m = l;
            for e in t[i].range_mut(l + 1..r) {
                *e.1 += 1;
            }
        }
    }
    let mut s = 2000 * 2000;
    for i in 0..2000 {
        for e in t[i] {
        }
    }
    println!("{}", n);
}
