fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n]
    }
    let mut h = std::collections::HashMap::new();
    for a in a {
        *h.entry(a).or_insert(0) += 1;
    }
    let mut c = 0;
    for (_, v) in h {
        if v > 1 {
            c += (v - 1) * v * (n - v) / 2;
        }
    }

    println!("{}", c);
}
