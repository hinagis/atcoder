use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {s: C}
    let n = s.len();
    let mut c = std::collections::HashSet::new();
    for l in 0..n {
        for r in l + 1..=n {
            c.insert(&s[l..r]);
        }
    }
    println!("{}", c.len());
}
