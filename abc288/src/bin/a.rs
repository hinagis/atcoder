use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {n: usize}
    for _ in 0..n {
        I! {
            a: i64,
            b: i64
        }
        println!("{}", a + b);
    }
}
