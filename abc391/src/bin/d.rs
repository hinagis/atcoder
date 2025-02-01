use std::usize;

use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        w: usize
    }
    let mut p = vec![vec![]; w];
    for i in 0..n {
        I! {
            x: U,
            y: usize
        }
        p[x].push((y, i));
    }
    let mut m = usize::MAX;
    for i in 0..w {
        m = m.min(p[i].len());
        p[i].sort_by(|a, b| a.0.cmp(&b.0));
    }
    for i in 0..w {
        p[i].drain(m..);
    }
    let mut l = vec![usize::MAX; n];
    let mut t = 0;
    for i in 0..m {
        for j in 0..w {
            t = t.max(p[j][i].0);
        }
        for j in 0..w {
            t = t.max(p[j][i].0);
        }
        for j in 0..w {
            l[p[j][i].1] = t.max(p[j][i].0);
        }
    }
    I! {q: usize}
    for _ in 0..q {
        I! {
            t: usize,
            a: U
        }
        println!("{}", if t < l[a] {"Yes"} else {"No"});
    }
}
