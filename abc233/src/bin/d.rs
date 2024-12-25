fn main() {
    proconio::input! {
        n: usize,
        k: i64,
        a: [i64; n]
    }

    let mut c = 0u64;
    let mut h = std::collections::HashMap::new();
    let mut s = 0;
    for a in a {
        *h.entry(s).or_insert(0) += 1;
        s += a;
        c += *h.get(&(s - k)).unwrap_or(&0);
    }

    println!("{}", c);
}
