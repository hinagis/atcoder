use omniswap::swap;
use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize,
    }
    let mut p = (0..n).collect::<Vec<_>>();
    let mut s = (0..n).map(|i| (i, i)).collect::<Vec<_>>();
    for _ in 0..q {
        I! {op: u8}
        if op == 3 {
            I! {a: U}
            println!("{}", s[p[a]].1 + 1);
        } else {
            I! {
                a: U,
                b: U
            }
            if op == 1 {
                p[a] = s[b].0;
            } else {
                let c = s[a].0;
                let d = s[b].0;
                swap!(&mut s[a].0, &mut s[b].0);
                swap!(&mut s[c].1, &mut s[d].1);
            }
        }
    }
}
