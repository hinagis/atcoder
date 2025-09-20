use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {t: u32}
    for _ in 0..t {
        I! {a: u32, b: u32, c: u32}
        println!("{}", a.min(c).min((a + b + c) / 3));
    }
}
