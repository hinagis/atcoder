fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n]
    }
    let mut b = Vec::with_capacity(n);
    let mut h = std::collections::HashMap::new();
    for (i, e) in a.iter().enumerate() {
        let r = if let Some(p) = h.get(e) {*p} else {-1};
        h.insert(*e, i as i32 + 1);
        b.push(r);
    }
    println!("{}", b.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(" "));
}
