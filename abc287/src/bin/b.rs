use proconio::{input as I};
fn main() {
    I! {
        n: usize,
        m: usize,
        mut s: [String; n]
    }
    let mut h = std::collections::HashSet::new();
    for _ in 0..m {
        I! {t: String}
        h.insert(t);
    }
    let mut c = 0;
    for i in 0..n {
        s[i].drain(0..3);
        if h.contains(&s[i]) {
            c += 1;
        }
    }
    println!("{}", c);
}
