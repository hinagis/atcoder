use itertools::Itertools;
use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        d: [u32; n - 1]
    }
    for i in 0..n - 1 {
        let mut r = vec![];
        let mut s = 0;
        for j in i..n - 1 {
            s += d[j];
            r.push(s);
        }
        println!("{}", r.iter().join(" "));
    }
}
