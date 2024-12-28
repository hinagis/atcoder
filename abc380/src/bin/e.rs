use std::iter::FromIterator;

use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize
    }
    let mut t = vec![1; n];
    let mut g = std::collections::BTreeMap::from_iter((0..n).map(|i| (i, i)));
    for _ in 0..q {
        I! {k: u8}
        if k == 1 {
            I! {
                x: U,
                c: U
            }
            let (&i, &p) = g.range(..=x).next_back().unwrap();
            let d = if let Some((&r, &p)) = g.range(i + 1..).next() {
                let d = r - i;
                if p == c {
                    g.remove(&r);
                }
                d
            } else {
                n - i
            };

            t[p] -= d;
            t[c] += d;
            *g.get_mut(&i).unwrap() = c;

            if let Some(l) = g.range(..i).next_back() {
                if *l.1 == c {
                    g.remove(&i);
                }
            }
        } else {
            I! {c: U}
            println!("{}", t[c]);
        }
    }
}
