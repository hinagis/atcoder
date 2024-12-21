use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {n: u32}
    for x in 0..=n {
        for y in 0..=n - x {
            for z in 0..=n - x - y {
                println!("{x} {y} {z}");
            }
        }
    }
}
