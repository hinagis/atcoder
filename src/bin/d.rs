fn main() {
    proconio::input! {
        _: i32,
        _: i32,
        p: [(i32, i32)],
        a: [i32],
        b: [i32]
    }
    let mut m = std::collections::HashMap::new();
    for &(p, q) in &p {
        let p = a.binary_search(&p).unwrap_err();
        let q = b.binary_search(&q).unwrap_err();
        *m.entry((p, q)).or_insert(0) += 1;
    }
    let l = if m.len() < (a.len() + 1) * (b.len() + 1) {0} else {*m.values().min().unwrap()};
    println!("{} {}", l, *m.values().max().unwrap())
}
