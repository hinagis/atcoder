use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {s: [String]}
    for s in s.iter().rev() {
        println!("{}", s);
    }
}
