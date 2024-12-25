use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {t: usize}
    for _ in 0..t {
        I! {a: [u32]}
        println!("{}", a.iter().filter(|&&c| c % 2 != 0).count());
    }
}
