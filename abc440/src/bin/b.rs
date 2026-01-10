use itertools::Itertools;
use proconio::input as I;
fn main() {
    I! {t: [u32]}
    println!("{}", t.iter().enumerate().sorted_by(|a, b| a.1.cmp(b.1)).take(3).map(|(i, _)| i + 1).join(" "));
}
