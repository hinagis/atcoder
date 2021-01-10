use proconio::{input, marker::Usize1};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        c: i64,
        abc: [(Usize1, Usize1, i64); n]
    }
    let mut v = HashMap::new();
    let mut d = Vec::with_capacity(2 * n);
    for &(a, b, c) in &abc {
        v.entry(a).or_insert((0, 0)).0 += c;
        v.entry(b).or_insert((0, 0)).1 += c;
        d.push(a);
        d.push(b);
    }
    d.sort();
    d.dedup();
    let mut r = 0;
    let mut nc = 0;
    let mut p = 0;
    for i in d {
        r += (i - p) as i64 * if nc < c {nc} else {c};
        p = i + 1;
        if let Some(c) = v.get(&i) {
            nc += c.0
        }
        r += if nc < c {nc} else {c};
        if let Some(c) = v.get(&i) {
            nc -= c.1
        }
    }

    println!("{}", r);
}
