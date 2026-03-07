use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: u32,
        mut x: u32,
    }
    for _ in 0..n {
        I! {a: u32}
        println!("{}", if a < x {
            x = a;
            1
        } else {
            0
        });
    }
}
