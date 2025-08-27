use itertools::Itertools;
use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        q: usize,
        mut a: [u64; n],
        mut b: [u64; n],
        v: [(char, U, u64); q]
    }
    let mut r = Vec::with_capacity(n);
    let mut s = (0..n).fold(0, |s, i| s + a[i].min(b[i]));
    for (c, x, v) in v {
        s -= a[x].min(b[x]);
        s += if c == 'A' {
            a[x] = v;
            if v < b[x] {v} else {b[x]}
        } else {
            b[x] = v;
            if v < a[x] {v} else {a[x]}
        };
        r.push(s);
    }

    println!("{}", r.iter().join("\n"));
}
