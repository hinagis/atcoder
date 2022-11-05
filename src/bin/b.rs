use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }
    let mut h = std::collections::HashMap::new();
    for &(a, b) in &ab {
        h.entry(a).or_insert(vec![]).push(b);
        h.entry(b).or_insert(vec![]).push(a);
    }
    for i in 1..=n {
        let e = h.entry(i).or_insert(vec![]);
        e.sort();
        println!("{} {}", e.len(), e.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" "));
    }
}
