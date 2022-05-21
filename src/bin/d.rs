fn main() {
    proconio::input! {
        n: usize,
        a: [u32; n]
    }
    let mut c = vec![std::collections::HashMap::new(); n];
    let mut h = std::collections::HashMap::new();
    for i in (0..n).rev() {
        *h.entry(a[i]).or_insert(0) += 1;
        c[i] = h.clone();
    }
    let mut r = 0;
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            if a[i] != a[j] {
                let v = c[j].get(&a[i]).unwrap_or(&0);
                let w = c[j].get(&a[j]).unwrap_or(&0) - 1;
                r += (n - 1 - j) as u64 - v - w;
            }
        }
    }
    println!("{}", r);
}
