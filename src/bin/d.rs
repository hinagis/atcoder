use std::collections::HashMap as H;

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        lr: [(usize, usize); n]
    }

    let mut a = H::new();
    let mut b = H::new();
    for &(l, r) in lr.iter() {
        let p = a.entry(l).or_insert(r);
        if r < *p {
            *p = r;
        }
        let p = b.entry(r).or_insert(l);
        if l > *p {
            *p = r;
        }
    }
    let mut c = a.clone().iter().map(|(l, r)| (*r, *l)).collect::<Vec<_>>();
    c.sort();

    let mut t = m * (m + 1) / 2;
    for (&r, &l) in b.iter() {
        let s = c.binary_search(&(r, 0)).unwrap_err();
        t += s + 1;
        t -= l * (l + 1) / 2;
    }
    for (_, &r) in a.iter() {
        let n = m - r;
        t += 1;
        t -= n * (n + 1) / 2;
    }
    println!("{}", t);
}
