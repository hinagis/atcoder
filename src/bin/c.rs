fn main() {
    proconio::input! {
        n: usize,
        d: [(u64, u64); n]
    }
    let mut h = std::collections::HashMap::new();
    for (a, c) in d {
        let h = h.entry(c).or_insert(u64::MAX);
        *h = a.min(*h);
    }
    println!("{}", h.values().max().unwrap());
}
