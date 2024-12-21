use std::u64;
use std::collections::{VecDeque as Q, HashSet as H, HashMap as M};

use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        m: usize,
    }
    let mut g = vec![M::new(); n];
    for _ in 0..m {
        I! {
            u: U,
            v: U,
            t: u64
        }
        let p = g[u].entry(v).or_insert(u64::MAX);
        if t < *p {
            *p = t;
        }
        let p = g[v].entry(u).or_insert(u64::MAX);
        if t < *p {
            *p = t;
        }
    }
    I! {q: usize}
    for _ in 0..q {
        I! {
            k: usize,
            b: [U; k]
        }

        let mut m = u64::MAX;
        let mut q = Q::new();
        q.push_back((0, 0, vec![false; k], H::new()));
        while let Some((i, s, mut f, mut h)) = q.pop_front() {
            if s >= m {continue}
            if h.contains(&i) {continue}
            h.insert(i);

            for j in 0..k {
                if b[j] == i {
                    if !f[j] {
                        h = H::new();
                        h.insert(i);
                    }
                    f[j] = true;
                }
            }
            if i == n - 1 && f.iter().all(|&e| e) {
                m = m.min(s);
                continue;
            }

            for (&j, &t) in g[i].iter() {
                q.push_back((j, s + t, f.clone(), h.clone()));
            }
        }
        println!("{}", m);
    }
}
