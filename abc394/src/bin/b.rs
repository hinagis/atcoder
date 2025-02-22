use itertools::Itertools;

fn main() {
    proconio::input! {mut s: [String]}
    s.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("{}", s.iter().join(""));
}
