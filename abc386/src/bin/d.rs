use std::collections::BTreeMap as T;
use proconio::input as I;

fn main() {
    I! {
        _: usize,
        m: usize,
    }
    let mut u = T::new();
    let mut v = T::new();
    let mut w = vec![];
    for _ in 0..m {
        I! {
            x: u64,
            y: u64,
            c: char
        }
        if c == 'B' {
            let u = u.entry(x).or_insert(y);
            *u = y.max(*u);
            let v = v.entry(y).or_insert(x);
            *v = x.max(*v);
        } else {
            w.push((x, y));
        }
    }
    let mut t = 0;
    for u in u.iter_mut().rev() {
        t = t.max(*u.1);
        *u.1 = t;
    }
    t = 0;
    for v in v.iter_mut().rev() {
        t = t.max(*v.1);
        *v.1 = t;
    }
    for (x, y) in w {
        if let Some((_, &l)) = u.range(x..).next() {
            if y <= l {
                println!("No");
                return;
            }
        }
        if let Some((_, &l)) = v.range(y..).next() {
            if x <= l {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
