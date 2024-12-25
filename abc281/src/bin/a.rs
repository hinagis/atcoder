use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {n: usize}
    for i in (0..=n).rev() {
        println!("{}", i);
    }
}
