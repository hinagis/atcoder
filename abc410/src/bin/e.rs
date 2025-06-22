use proconio::input as I;
use std::collections::BTreeSet as T;

fn main() {
    I! {
        n: usize,
        h: u32,
        m: u32
    }
    let mut p = T::new();
    p.insert((h, m));
    for i in 0..n {
        let mut t = T::new();
        I! {
            a: u32,
            b: u32
        }
        for &(h, m) in &p {
            if h >= a {
                t.insert((h - a, m));
            }
            if m >= b {
                t.insert((h, m - b));
            }
        }
        if t.is_empty() {
            println!("{}", i);
            return;
        }
        p = t;
    }
    println!("{}", n);
}
