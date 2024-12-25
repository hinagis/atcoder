use proconio::{input as I, fastout as F, marker::Usize1 as U1};
use std::collections::HashSet as H;

#[F]
fn main() {
    I! {
        n: usize,
        a: [u32; n],
        b: [u32; n],
    }

    let mut ac = vec![0; n];
    let mut bc = vec![0; n];
    let mut ak = Vec::with_capacity(n);
    let mut bk = Vec::with_capacity(n);
    let mut sa = H::new();
    let mut sb = H::new();
    for i in 0..n {
        if sa.insert(a[i]) {ak.push(a[i]);}
        ac[i] = sa.len();
        if sb.insert(b[i]) {bk.push(b[i]);}
        bc[i] = sb.len();
    }

    let mut f = vec![false; n + 1];
    let mut s = H::new();
    for i in 0..(ak.len().min(bk.len())) {
        if !s.remove(&ak[i]) {s.insert(ak[i]);}
        if !s.remove(&bk[i]) {s.insert(bk[i]);}
        f[i + 1] = s.len() == 0;
    }

    I! {q: usize}
    for _ in 0..q {
        I! {x: U1, y: U1}
        println!("{}", if ac[x] == bc[y] && f[ac[x]] {"Yes"} else {"No"});
    }
}
