use proconio::{input as I, marker::Bytes as B};

fn main() {
    I! {s: B}

    let mut h = std::collections::HashMap::new();
    h.insert(0, 1u64);
    let mut c = 0;
    for &b in &s {
        c ^= 1 << (b - b'0');
        *h.entry(c).or_default() += 1;
    }
    
    println!("{}", h.values().fold(0, |s, v| s + (v * (v - 1)) / 2));
}
