use proconio::{input as I, fastout as F};
use std::collections::{VecDeque as V, HashSet as H};

#[F]
fn main() {
    I! {
        n: usize,
        m: usize
    }

    let mut t = vec![vec![]; n + 1];
    for _ in 0..m {
        I! {
            a: usize,
            b: usize
        }
        t[a].push(b);
        t[b].push(a);
    }

    I! {q: usize}
    for _ in 0..q {
        I! {
            x: usize,
            k: usize
        }

        let mut r = 0;
        let mut h = H::new();
        h.insert(x);
        let mut q = V::new();
        q.push_back((x, 0));
        while let Some((x, c)) = q.pop_front() {
            r += x;
            if c < k {
                for &e in &t[x] {
                    if h.insert(e) {
                        q.push_back((e, c + 1));
                    }
                }
            }
        }
        println!("{}", r);
    }
}
