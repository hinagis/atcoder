use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        v: [(u64, u64, u64); n],
        q: u32,
    }
    for _ in 0..q {
        I! {x: u64}
    }
    println!("{}", n);
}
