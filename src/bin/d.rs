fn main() {
    proconio::input! {z: [(u64, u64)]}
    let mut d = std::collections::BTreeMap::new();
    for (l, r) in z {
        (*d.entry(l).or_insert((0, 0))).0 += 1;
        (*d.entry(r).or_insert((0, 0))).1 += 1;
    }
    let mut r = 0i64;
    let mut h = 0;
    for &(p, n) in d.values() {
        r += h * p;
        r += p * (p - 1) / 2;
        h += p;
        h -= n;
    }
    println!("{}", r);
}
