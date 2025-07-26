use proconio::{input as I, fastout as F};
use std::collections::{VecDeque as Q, BTreeSet as S};
use itertools::Itertools;

#[F]
fn main() {
    I! {t: u32}
    for _ in 0..t {
        I! {
            n: usize,
            s: String
        }
        let p = s.chars().enumerate().filter(|&(_, c)| c == '1').map(|(i, _)| i + 1).sorted().collect::<S<_>>();
        let mut q = Q::new();
        let mut r = "No";
        q.push_back(0);
        while let Some(e) = q.pop_front() {
            if e == (1 << n) - 1 {
                r ="Yes";
                break;
            }
            for i in 0..n {
                if e & (1 << i) != 0 {continue}
                let b = e | (1 << i);
                if p.contains(&b) {continue}
                q.push_back(b);
            }
        }
        println!("{}", r);
    }
}
