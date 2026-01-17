use itertools::Itertools;
use proconio::{input as I, marker::Usize1 as U};
use std::collections::{VecDeque as Q, HashSet as S};

fn main() {
    I! {
        n: usize,
        m: usize,
        l: u64,
        s: u64,
        t: u64
    }
    let mut e = vec![vec![]; n];
    for _ in 0..m {
        I! {
            u: U,
            v: U,
            c: u64
        }
        e[u].push((v, c));
    }

    let mut r = S::new();
    let mut q = Q::new();
    q.push_back((0, 0, 0));
    while let Some((u, i, w)) = q.pop_front(){
        let i = i + 1;
        if i == l {
            for &(v, c) in &e[u] {
                let w = w + c;
                if w >= s && w <= t {
                    r.insert(v + 1);
                }
            }
        } else {
            for &(v, c) in &e[u] {
                q.push_back((v, i, w + c));
            }
        }
    }
    let mut r = r.iter().collect_vec();
    r.sort();
    println!("{}", r.iter().join(" "));
}
