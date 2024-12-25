use proconio::{input, fastout};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }
    let a: BTreeSet<_> = a.into_iter().collect();
    let b: BTreeSet<_> = b.into_iter().collect();

    for c in a.intersection(&b) {
        println!("{}", c);
    }
}
