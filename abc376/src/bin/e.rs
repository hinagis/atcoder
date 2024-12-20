use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {t: usize}
    for _ in 0..t {
        I! {
            n: usize,
            k: usize,
            a: [u64; n],
            b: [u64; n],
        }
        let mut h = std::collections::BTreeMap::new();
        let mut s = 0;
        for i in 0..k {
            *h.entry(a[i]).or_insert(0u32) += 1;
            s += b[i];
        }
        let mut m = h.keys().last().unwrap() * s;
        for i in k..n {
            let p = h.entry(a[i - k]).or_default();
            if *p > 1 {
                *p -= 1;
            } else {
                h.remove(&a[i - k]);
            }
            *h.entry(a[i]).or_default() += 1;
            s -= b[i - k];
            s += b[i];
            m = m.min(h.keys().last().unwrap() * s);
        }
        println!("{}", m);
    }
}
