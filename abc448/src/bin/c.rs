use itertools::Itertools;
use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize,
        a: [u32; n]
    }
    let w = a.iter().enumerate().sorted_by(|x, y| x.1.cmp(y.1)).collect_vec();
    for _ in 0..q {
        I! {k: u8}
        let mut f = vec![true; n];
        for _ in 0..k {
            I! {b: U}
            f[b] = false;
        }
        for i in 0.. {
            if f[w[i].0] {
                println!("{}", w[i].1);
                break;
            }
        }
    }
}
