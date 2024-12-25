use proconio::{input as I, fastout as F};
use std::collections::HashMap as H;

#[F]
fn main() {
    I! {
        h: u32,
        w: u32,
        mut y: u32,
        mut x: u32,
        n: usize,
        b: [(u32, u32); n],
        q: usize,
    }

    let mut r = H::new();
    let mut c = H::new();
    for &(i, j) in &b {
        r.entry(i).or_insert(vec![]).push(j);
        c.entry(j).or_insert(vec![]).push(i);
    }
    for r in r.values_mut() {
        r.sort();
    }
    for c in c.values_mut() {
        c.sort();
    }

    for _ in 0..q {
        I! {
            d: char,
            l: u32,
        }
        if d == 'U' {
            if let Some(c) = c.get(&x) {
                let p = c.binary_search(&y).unwrap_err();
                if p == 0 {
                    if y > l {
                        y = y - l
                    } else {
                        y = 1
                    }
                } else {
                    if y > l {
                        y = (c[p - 1] + 1).max(y - l)
                    } else {
                        y = c[p - 1] + 1
                    }
                }
            } else {
                if y > l {
                    y = y - l
                } else {
                    y = 1
                }
            }
        } else if d == 'D' {
            if let Some(c) = c.get(&x) {
                let p = c.binary_search(&y).unwrap_err();
                if p >= c.len() {
                    y = h.min(y + l)
                } else {
                    y = (c[p] - 1).min(y + l)
                }
            } else {
                y = h.min(y + l)
            }
        } else if d == 'L' {
            if let Some(r) = r.get(&y) {
                let p = r.binary_search(&x).unwrap_err();
                if p == 0 {
                    if x > l {
                        x = x - l
                    } else {
                        x = 1
                    }
                } else {
                    if x > l {
                        x = (r[p - 1] + 1).max(x - l)
                    } else {
                        x = r[p - 1] + 1
                    }
                }
            } else {
                if x > l {
                    x = x - l
                } else {
                    x = 1
                }
            }
        } else {
            if let Some(r) = r.get(&y) {
                let p = r.binary_search(&x).unwrap_err();
                if p >= r.len() {
                    x = w.min(x + l)
                } else {
                    x = (r[p] - 1).min(x + l)
                }
            } else {
                x = w.min(x + l)
            }
        }
        println!("{} {}", y, x);
    }
}
