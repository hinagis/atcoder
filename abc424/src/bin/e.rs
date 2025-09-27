use proconio::{input as I, fastout as F};
use std::collections::VecDeque as Q;

#[F]
fn main() {
    I!(t: u32);
    for _ in 0..t {
        I! {
            n: usize,
            mut k: usize,
            x: usize,
            mut a: [f64; n],
        }

        a.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let mut q = Q::new();
        q.push_back((a[0] * 0.5, 2));
        let mut p = a[1..].iter().map(|&a| (a, 1)).collect::<Q<_>>();
        k -= 1;
        while k > 0 {
            let (l, c) = if p.len() > 0 && p[0].0 >= q[0].0 {
                p.pop_front().unwrap()
            } else {
                q.pop_front().unwrap()
            };
            let t = c.min(k);
            q.push_back((l * 0.5, 2 * t));
            if c > t {
                q.push_front((l, c - t));
            }
            k -= t;
        }

        let mut a = p.into_iter().chain(q).collect::<Vec<_>>();
        a.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let mut s = 0;
        for (len, c) in a.into_iter() {
            s += c;
            if s >= x {
                println!("{}", len);
                break;
            }
        }
    }
}
