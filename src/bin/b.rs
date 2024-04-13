use std::collections::HashMap as H;

fn main() {
    proconio::input! {s: String}
    let mut a = H::new();
    for c in s.chars() {
        *a.entry(c).or_insert(0) += 1;
    }
    let mut c = H::new();
    for &b in a.values() {
        *c.entry(b).or_insert(0) += 1;
    }
    println!("{}", if c.values().any(|&c| c != 0 && c != 2) {"No"} else {"Yes"});
}
