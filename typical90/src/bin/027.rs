use proconio::{input, fastout};

#[fastout]
fn main() {
    let mut h = std::collections::HashSet::new();
    input! {n: usize}
    for i in 1..=n {
        input! {s: String}
        if ! h.contains(&s) {
            h.insert(s);
            println!("{}", i);
        }
    }
}
