fn main() {
    proconio::input! {
        n: usize,
        a: [u32; n]
    }
    let mut h = std::collections::HashMap::new();
    h.insert(a[0] ^ a[1], 1);
    for i in 2..n {
        for &c in h.keys() {
            *h.entry(c ^ a[i]).or_insert(0) += 1;
        }
    }
    println!("{}", h.iter().fold(0, |s, (k, v)| s + k * v));
}
