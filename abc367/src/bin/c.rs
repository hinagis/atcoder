use proconio::{input as I, fastout as F};
use itertools::Itertools;

#[F]
fn main() {
    I! {
        n: usize,
        k: u8,
        r: [u8; n]
    }
    let mut v = vec![1; n];
    let mut i = n - 1;
    loop {
        if v.iter().sum::<u8>() % k == 0 {
            println!("{}", v.iter().join(" "));
        }
        v[i] += 1;
        while v[i] > r[i] {
            v[i] = 1;
            if i > 0 {
                i -= 1;
            } else {
                return;
            }
            v[i] += 1;
        }
        i = n - 1;
    }
}
