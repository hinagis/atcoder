use std::iter::FromIterator;

use itertools::Itertools;
use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        k: usize,
        s: C
    }
    let mut h = std::collections::HashMap::new();
    for i in 0..=n - k {
        *h.entry(&s[i..i + k]).or_insert(0) += 1;
    }
    let m = h.values().max().unwrap();
    let v = h.iter().filter(|(_, v)| *v == m).map(|(k, _)| *k).collect::<Vec<_>>();
    println!("{m}\n{}", v.iter().map(|s| String::from_iter(s.iter())).sorted().join(" "));
}
