use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut t = FenwickTree::from_vec(a);
    for _ in 0..q {
        input! {
            f: u8,
            l: usize,
            r: usize,
        };
        if f == 0 {
            t.add(l, r);
        } else {
            println!("{}", t.cum(l, r));
        }
    }
}

struct FenwickTree<T> {
    v: Vec<T>,
}

use std::ops::AddAssign;
use num::{Integer, zero};

impl<T: Copy + AddAssign + Integer> FenwickTree<T> {
    fn new(n: usize) -> Self {
        Self {
            v: vec![zero(); n],
        }
    }

    fn from_vec(v: Vec<T>) -> Self {
        let n = v.len();
        let mut t = Self::new(n);
        for i in 0..n {
            let v = t.v[i] + v[i];
            t.v[i] = v;
            let p = i + Self::lsb(i + 1);
            if p < n {
                t.v[p] += v;
            }
        }
        t
    }

    fn add(&mut self, mut i: usize, v: T) {
        while i < self.v.len() {
            self.v[i] += v;
            i += Self::lsb(i + 1);
        }
    }

    fn cum(&self, l: usize, r: usize) -> T {
        assert!(l <= r && r <= self.v.len());

        self.cum0(r) - self.cum0(l)
    }

    fn cum0(&self, mut r: usize) -> T {
        let mut v = zero();
        while r > 0 {
            v += self.v[r - 1];
            r -= Self::lsb(r);
        }
        v
    }

    fn lsb(i: usize) -> usize {
        i & i.wrapping_neg()
    }
}
