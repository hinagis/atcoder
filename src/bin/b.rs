use proconio::{input as I, fastout as F};
use itertools::Itertools;
use std::collections::{HashMap as H, BTreeMap as T, BTreeSet as B};

#[F]
fn main() {
    I! {n: usize}
    let mut h = H::new();
    for i in 1..=n {
        I! {
            c: usize,
            a: [usize; c]
        }
        for j in 0..c {
            h.entry(a[j]).or_insert(T::new())
                .entry(c).or_insert(B::new())
                .insert(i);
        }
    }
    I! {x: usize}
    if let Some(c) = h.get(&x) {
        let (_, b) = c.first_key_value().unwrap();
        println!("{}\n{}", b.len(), b.iter().join(" "));
    } else {
        println!("0");
    }
}
