use itertools::Itertools;
use proconio::input as I;
use std::collections::{BTreeMap as T, VecDeque as Q};

fn main() {
    I! {
        n: usize,
        k: u64,
    }

    let mut m = T::new();
    for i in 0..n {
        I! {
            a: u64,
            b: u64,
            c: u64
        }
        m.entry(a).or_insert((vec![], 0)).0.push((i, b, c));
    }

    let mut r = vec![0; n];

    let mut e = 0;
    let mut q = Q::new();
    while let Some((a, v)) = m.pop_first() {
        for &(i, b, c) in v.0.iter() {
            q.push_back((i, b, c));
        }
        e -= v.1;
        while let Some((i, b, c)) = q.pop_front() {
            if e + c > k {
                q.push_front((i, b, c));
                break;
            }
            (*m.entry(a + b).or_insert((vec![], 0))).1 += c;
            r[i] = a;
            e += c;
        }
    }
    println!("{}", r.iter().join("\n"));
}
