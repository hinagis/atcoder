use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        b: [(u64, u64); n],
        q: usize
    }
    for _ in 0..q {
        I! {
            t: U,
            d: u64
        }
        let (q, r) = b[t];
        let m = d % q;
        println!("{}", d + r + if m > r {q} else {0} - m);
    }
}
