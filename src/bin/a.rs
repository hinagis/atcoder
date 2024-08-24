use itertools::Itertools;
use proconio::input as I;

fn main() {
    I! {
        n: usize,
        k: usize,
        a: [u32; n - k],
        b: [u32; k]
    }
    println!("{} {}", b.iter().join(" "), a.iter().join(" "));
}
