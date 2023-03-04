use proconio::{input as I, marker::Usize1 as U};
use std::collections::{VecDeque as V, HashSet as H};

fn main() {
    I! {
        n: usize,
        e: [(U, U)],
    }

    let mut q = V::new();
    let mut t = vec![H::new(); n];
    for &(u, v) in &e {
        t[u].insert(v);
        q.push_back((u, v));
    }
    let mut r = 0;
    while let Some((u, v)) = q.pop_front() {
        for j in t[v].clone() {
            if u != j && t[u].insert(j) {
                q.push_back((u, j));
                r += 1;
            }
        }
    }
    println!("{}", r);
}
