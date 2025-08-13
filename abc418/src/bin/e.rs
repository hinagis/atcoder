use num_integer::gcd;
use std::collections::HashMap as H;

fn main() {
    proconio::input! {
        n: usize,
        p: [(i64, i64); n]
    }
    let mut s = H::new();
    let mut m = H::new();
    for i in 0..n {
        for j in i + 1..n {
            let mut u = p[i].0 - p[j].0;
            let mut v = p[i].1 - p[j].1;
            if u == 0 {
                v = 1;
            } else if v == 0 {
                u = 1;
            } else {
                if u < 0 {
                    u = -u;
                    v = -v;
                }
                let g = gcd(u.abs(), v.abs());
                u /= g;
                v /= g;
            }
            *s.entry((u, v)).or_insert(0) += 1;
            *m.entry((p[i].0 + p[j].0, p[i].1 + p[j].1)).or_insert(0) += 1;
        }
    }
    println!("{}", calc(&s) - calc(&m));
}

fn calc(v: &H<(i64, i64), i64>) -> i64 {
    v.iter().fold(0, |s, (_, c)| s + c * (c - 1) / 2)
}