fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }
    let mut s = std::collections::HashMap::new();
    let mut t = 0;
    for i in 0..n {
        t += a[i];
        t %= m;
        *s.entry(t).or_insert(0) += 1;
    }
    let mut t = if let Some(v) = s.get(&0) {*v} else {0};
    for i in 1..n {
        if let Some(v) = s.get(&i) {
            t += *v;
        }
    }
    println!("{}", t);
}
