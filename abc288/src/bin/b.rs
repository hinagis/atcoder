use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        k: usize
    }
    let mut d = Vec::with_capacity(k);
    for _ in 0..k {
        I! {s: String}
        d.push(s);
    }
    d.sort();
    for _ in k..n {
        I! {_: String}
    }
    for s in &d {
        println!("{}", s);
    }
}
