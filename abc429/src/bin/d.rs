use std::iter::FromIterator;

use proconio::input as I;

fn main() {
    I! {
        n: usize,
        m: usize,
        c: usize,
        a: [usize; n]
    }

    if n == 1 {
        println!("{}", m);
        return;
    }

    let mut b = std::collections::BTreeMap::new();
    for &a in a.iter() {
        *b.entry(a).or_insert(0) += 1;
    }

    let v = Vec::from_iter(b.iter());
    let mut r = 0;
    let mut k = 0;
    while k < c {
        k += v[r].1;
        r = (r + 1) % v.len();
    }
    let mut s = (v[0].0 + m - v[v.len() - 1].0) * k;
    k -= v[0].1;

    for x in 1..v.len() {
        while k < c {
            k += v[r].1;
            r = (r + 1) % v.len();
        }
        s += (v[x].0 - v[x - 1].0) * k;
        k -= v[x].1;
    }
    println!("{}", s);
}
