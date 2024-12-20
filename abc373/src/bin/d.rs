use itertools::Itertools;
use proconio::{input as I, marker::Usize1 as U};
use std::{collections::{BTreeSet as T, VecDeque as Q}, iter::FromIterator};

fn main() {
    I! {
        n: usize,
        e: [(U, U, i64)],
    }
    let mut r = T::from_iter(0..n);
    let mut t = vec![vec![]; n];
    for &(u, v, w) in e.iter() {
        t[u].push((v, w, true));
        t[v].push((u, w, false));
    }
    let mut v = vec![0; n];
    while let Some(h) = r.pop_first() {
        let mut q = Q::new();
        q.push_back(h);
        r.remove(&h);
        while let Some(i) = q.pop_front() {
            for &(j, w, d) in t[i].iter() {
                if r.contains(&j) {
                    q.push_back(j);
                    r.remove(&j);
                    v[j] = v[i] + w * if d {1} else {-1};
                }
            }
        }
    }
    println!("{}", v.iter().join(" "));
}
