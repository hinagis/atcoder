use itertools::Itertools;
use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        k: usize,
        a: [u32; n],
    }
    let mut h = std::collections::HashMap::new();
    for i in 0..n {
        let mut g = 1;
        let mut b = a[i];
        let mut d = 2;
        while d * d <= b {
            while b % d == 0 {
                b /= d;
                g *= d;
                *h.entry(g).or_insert(0) += 1;
            }
            d += 1;
        }
        if d > 1 {
            g *= b;
            *h.entry(g).or_insert(0) += 1;
        }
    }
    dbg!(&h);
    for i in 0..n {
        let mut g = vec![1; 1];
        let mut b = a[i];
        let mut d = 2;
        while d * d <= b {
            while b % d == 0 {
                for j in 0..g.len() {
                    g.push(g[j] * d);
                }
                b /= d;
            }
            d += 1;
        }
        if d > 1 {
            for j in 0..g.len() {
                g.push(g[j] * b);
            }
        }
        let mut r = 1;
        g.iter().unique().for_each(|&x| {
            if let Some(&c) = h.get(&x) {
                if c >= k {
                    r = r.max(x);
                }
            }
        });
        println!("{}", r);
    }
}
