use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut l = 0;
    let mut p = 1;
    while p < n + m {
        l += 1;
        p *= 2;
    }

    // 565042129 is Mint(1) / pow(MInt(2), 20)
    let z = pow(MInt(565042129), pow(2, 20 - l));

    let make = |d: Vec<usize>, n| {
        let mut v = Vec::with_capacity(p);
        for i in 0..n { v.push(MInt(d[i])) }
        for _ in n..p { v.push(MInt(0)) }
        ntt(&v, 0, 1, z)
    };
    let a = make(a, n);
    let b = make(b, m);
    let c = {
        let mut v = Vec::with_capacity(p);
        for i in 0..p { v.push(a[i] * b[i]) }
        ntt(&v, 0, 1, MInt(1) / z)
    };

    print!("{}", (c[0] / MInt(p)).0);
    for i in 1..n + m - 1 {
        print!(" {}", (c[i] / MInt(p)).0);
    }
    println!();
}

use num::{pow, One};
use std::ops::{Add, Sub, Mul, Div};

const M: usize = 998244353;

#[derive(Clone, Copy, PartialEq, Debug)]
struct MInt(usize);

impl One for MInt {
    fn one() -> Self {
        return MInt(1);
    }
}

impl Add for MInt {
    type Output = Self;
    fn add(self, v: Self) -> Self {
        MInt((self.0 + v.0) % M)
    }
}

impl Sub for MInt {
    type Output = Self;
    fn sub(self, v: Self) -> Self {
        MInt((self.0 + M - v.0) % M)
    }
}

impl Mul for MInt {
    type Output = Self;
    fn mul(self, v: Self) -> Self {
        return MInt((self.0 * v.0) % M);
    }
}

impl Div for MInt {
    type Output = Self;
    fn div(self, v: Self) -> Self {
        assert!(v.0 != 0);
        let v = pow(v, M - 2);
        return self * v;
    }
}

fn ntt(
    v: &Vec<MInt>,
    s: usize,
    d: usize,
    z: MInt,
) -> Vec<MInt> {
    let n = v.len() / d;
    if n == 1 {
        vec![v[s]]
    } else {
        let g0 = ntt(v, s, d * 2, z * z);
        let g1 = ntt(v, s + d, d * 2, z * z);
        let mut p = MInt(1);
        let mut g = Vec::with_capacity(n);
        for _ in 0..2 {
            for i in 0..(n / 2) {
                g.push(g0[i] + p * g1[i]);
                p = p * z;
            }
        }
        g
    }
}
