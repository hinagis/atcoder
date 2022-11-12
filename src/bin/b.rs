use proconio::{input as I};
use regex::Regex;

fn main() {
    I! {n: usize}
    let p = Regex::new("[HDCS][A23456789TJQK]").unwrap();
    let mut h = std::collections::HashSet::new();
    for _ in 0..n {
        I! {s: String}
        if p.is_match(&s) && h.insert(s) {
            continue;
        }
        println!("No");
        return;
    }
    println!("Yes");
}
