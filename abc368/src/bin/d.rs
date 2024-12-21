use std::iter::FromIterator;
use std::collections::HashSet as H;

use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        k: usize,
        e: [(U, U); n - 1],
        v: [U; k]
    }
    let mut t = vec![vec![]; n];
    for &(a, b) in &e {
        t[a].push(b);
        t[b].push(a);
    }
    let h: H<usize> = H::from_iter(v.iter().cloned());
    fn calc(t: &Vec<Vec<usize>>, h: &H<usize>, p: usize, i: usize) -> u64 {
        let mut c = 0;
        for &j in t[i].iter() {
            if j != p {
                c += calc(t, h, i, j);
            }
        }
        if c > 0 {
            c + 1
        } else {
            if h.contains(&i) {
                1
            } else {
                0
            }
        }
    }
    let mut c = 1;
    for &i in t[v[0]].iter() {
        c += calc(&t, &h, v[0], i);
    }
    println!("{}", c);
}
