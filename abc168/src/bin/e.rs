use proconio::input;
use std::collections::BTreeMap;

const M: u64 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        d: [(i64, i64); n],
    }

    let mut z = 0;
    let mut t = BTreeMap::new();
    for e in d {
        if e.0 == 0 && e.1 == 0 {
            z += 1;
        } else {
            fn grouping(e: &(i64, i64)) -> (i64, i64) {
                let e = if e.0 < 0 { (-e.0, -e.1) } else { *e };
                let g = num::integer::gcd(e.0, e.1);
                (e.0 / g, e.1 / g)
            }
            let e = match e {
                _ if e.0 == 0 => (0, 1),
                _ if e.1 == 0 => (1, 0),
                _ => grouping(&e),
            };
            *t.entry(e).or_insert(0) += 1;
        }
    }
    z %= M;

    let mut ans = 1;
    while let Some((&k, &c)) = t.iter().next() {
        t.remove(&k);

        let c2 = modpow(2, c);
        let inv = if k.1 < 0 { (-k.1, k.0) } else { (k.1, -k.0) };
        ans *= if let Some(c) = t.remove(&inv) { c2 + modpow(2, c) - 1 } else { c2 };
        ans %= M;
    }
    ans += z + M - 1;
    ans %= M;
    println!("{}", ans);
}

fn modpow(x: u64, y: u64) -> u64 {
    let mut result = 1;
    let mut a = x;
    let mut b = y;
    while b > 0 {
        if b % 2 == 1 {
            result = result * a % M;
        }
        a = a * a % M;
        b /= 2;
    }
    result
}
