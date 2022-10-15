use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        mut a: [u32; n]
    }
    a.sort_by(|a, b| b.cmp(a));
    let mut k = 0;
    let mut p = a[0];
    let mut h = std::collections::HashMap::new();
    for &a in &a {
        if a != p {
            k += 1;
            p = a;
        }
        *h.entry(k).or_insert(0) += 1;
    }

    for k in 0..n {
        println!("{}", if let Some(&e) = h.get(&k) {e} else {0});
    }
}
