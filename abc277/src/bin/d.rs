use std::collections::HashMap as H;

fn main() {
    proconio::input! {
        n: usize,
        m: u64,
        a: [u64; n],
    }
    let mut h = H::new();
    for i in 0..n {
        *h.entry(a[i]).or_insert(0) += 1;
    }
    let t = h.iter().fold(0, |s, (x, &v)| s + x * v);

    let mut s = H::new();
    for i in 0..n {
        if ! s.contains_key(&a[i]) {
            let v = calc(&a, m, &mut h, &mut s, a[i]);
            s.insert(a[i], v);
        }
    }
    println!("{}", t - s.iter().fold(0, |m, (_, &v)| m.max(v)));
}

fn calc(a: &Vec<u64>, m: u64, h: &mut H<u64, u64>, s: &mut H<u64, u64>, x: u64) -> u64 {
    if let Some(&v) = s.get(&x) {
        v
    } else if let Some(&v) = h.get(&(x)) {
        s.insert(x, 0);
        let c = calc(a, m, h, s, (x + 1) % m);
        s.entry(x).or_insert(c);
        x * v + c
    } else {
        0
    }
}
