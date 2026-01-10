use std::iter::FromIterator;

use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        q: u32,
        a: [usize; n]
    }
    let a = std::collections::BTreeSet::from_iter(a);
    for _ in 0..q {
        I! {
            x: usize,
            y: usize
        }
        let mut l = x + y - 1;
        let mut r = x + y + n + 1;
        while l < r {
            let m = (l + r) / 2;
            let c = a.range(x..=m).count();
            if m + 1 - x - c < y {
                l = m + 1;
            } else {
                r = m;
            }
        }
        println!("{}", l);
    }
}
