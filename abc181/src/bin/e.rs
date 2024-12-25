use std::cmp::{Ord, Ordering::Less};

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        mut h: [i32; n],
        mut w: [i32; m],
    }
    h.sort();

    let mut cl = vec![0; (n + 1) / 2];
    let mut cr = vec![0; (n + 1) / 2];
    for i in 1..((n + 1) / 2) {
        cl[i] = cl[i - 1] + h[i * 2 - 1] - h[i * 2 - 2];
        let j = (n + 1) / 2 - i;
        cr[j - 1] = cr[j] + h[j * 2] - h[j * 2 - 1];
    }
    let mut r = std::i32::MAX;
    for &w in &w {
        let i = lower_bound(&h, &w);
        let i = if i % 2 == 1 {i - 1} else {i};
        r = r.min(cl[i / 2] + cr[i / 2] + (h[i] - w).abs());
    }
    println!("{}", r);
}

fn lower_bound<T: Ord>(v: &Vec<T>, x: &T) -> usize {
    let mut l = 0;
    let mut r = v.len();

    while l != r {
        let m = (l + r) / 2;
        if v[m].cmp(x) == Less {
            l = m + 1
        } else {
            r = m
        }
    }
    l
}
