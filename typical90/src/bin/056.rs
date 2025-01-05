use std::{collections::{HashMap as H, VecDeque as V}, str::FromStr};
use proconio::input as I;

fn main() {
    I! {
        n: usize,
        s: usize
    }
    let mut h = vec![H::new(); s + 1];
    h[0].insert(0, (0, '_'));
    for i in 0..n {
        I! {
            a: usize,
            b: usize
        }
        let mut t = H::new();
        for (&k, _) in h[i].iter() {
            if k + a <= s {
                t.insert(k + a, (k, 'A'));
            }
            if k + b <= s {
                t.insert(k + b, (k, 'B'));
            }
        }
        h[i + 1] = t;
    }

    println!("{}", if let Some((mut k, c)) = h[n].get(&s) {
        let mut s = V::new();
        s.push_front(*c);
        for i in (1..n).rev() {
            let (p, c) = h[i].get(&k).unwrap();
            k = *p;
            s.push_front(*c);
        }
        s.iter().collect::<String>()
    } else {
        String::from_str("Impossible").unwrap()
    });
}
