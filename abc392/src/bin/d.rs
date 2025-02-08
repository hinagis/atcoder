use proconio::input as I;
// not cleared
fn main() {
    I! {n: usize}
    let mut h = vec![(0f64, std::collections::HashMap::new()); n];
    for i in 0..n {
        I! {k: usize}
        h[i].0 = k as f64;
        for _ in 0..k {
            I! {a: u64}
            *h[i].1.entry(a).or_insert(0) += 1;
        }
    }
    let mut m = 0f64;
    for i in 0..n {
        for j in i + 1..n {
            let mut s = 0;
            for (k, v) in &h[i].1 {
                s += h[j].1.get(k).unwrap_or(&0) * v;
            }
            m = m.max(s as f64 / (h[i].0 * h[j].0));
        }
    }
    println!("{}", m);
}
