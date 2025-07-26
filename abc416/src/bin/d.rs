use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {t: usize}
    for _ in 0..t {
        I! {
            n: usize,
            m: u64,
            mut a: [u64; n],
            mut b: [u64; n]
        }
        a.sort_by(|a, b| b.cmp(a));
        b.sort();
    }
}
