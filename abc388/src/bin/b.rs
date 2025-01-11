use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        d: usize,
        s: [(usize, usize); n]
    }
    for k in 1..=d {
        println!("{}", s.iter().map(|(t, l)| t * (l + k)).max().unwrap());
    }
}
