use proconio::{input as I, marker::Usize1 as U};
use std::collections::BTreeSet as T;

fn main() {
    I! {
        h: usize,
        w: usize,
        q: usize,
    }

    let mut t = T::new();
    for j in 0..w {
        t.insert(j);
    }
    let mut x = Vec::with_capacity(h);
    for _ in 0..h {
        x.push(t.clone());
    }
    let mut t = T::new();
    for i in 0..h {
        t.insert(i);
    }
    let mut y = Vec::with_capacity(w);
    for _ in 0..w {
        y.push(t.clone());
    }

    for _ in 0..q {
        I! {
            r: U,
            c: U
        }
        if x[r].contains(&c) {
            x[r].remove(&c);
            y[c].remove(&r);
        } else {
            let p = {
                let mut i = None;
                for &e in x[r].range(c..) {
                    i = Some(e);
                    break;
                }
                i
            };
            if let Some(p) = p {
                x[r].remove(&p);
                y[p].remove(&r);
            }
            let p = {
                let mut i = None;
                for &e in x[r].range(..c).rev() {
                    i = Some(e);
                    break;
                }
                i
            };
            if let Some(p) = p {
                x[r].remove(&p);
                y[p].remove(&r);
            }

            let p = {
                let mut i = None;
                for &e in y[c].range(r..) {
                    i = Some(e);
                    break;
                }
                i
            };
            if let Some(p) = p {
                x[p].remove(&c);
                y[c].remove(&p);
            }
            let p = {
                let mut i = None;
                for &e in y[c].range(..r).rev() {
                    i = Some(e);
                    break;
                }
                i
            };
            if let Some(p) = p {
                x[p].remove(&c);
                y[c].remove(&p);
            }
        }
    }
    println!("{}", x.iter().fold(0, |s, x| s + x.len()))
}
