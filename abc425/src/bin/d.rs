use proconio::{input as I, marker::Chars as C};
use std::collections::VecDeque as Q;

fn main() {
    I! {
        h: usize,
        w: usize,
        mut s: [C; h]
    }
    let mut v = vec![vec![true; w]; h];
    let mut q = Q::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                v[i][j] = false;
                q.push_back((i, j));
            }
        }
    }
    let mut f = true;
    while f {
        let mut d = Vec::with_capacity(q.len());
        while let Some((i, j)) = q.pop_front() {
            let c = (i > 0 && v[i - 1][j]) as u32 +
               (i < h - 1 && v[i + 1][j]) as u32 +
               (j > 0 && v[i][j - 1]) as u32 +
               (j < w - 1 && v[i][j + 1]) as u32;
            match c  {
                1 => d.push((i, j)),
                _ => ()
            }
        }
        f = d.len() > 0;
        let mut p = Q::with_capacity(q.len());
        for &(i, j) in d.iter() {
            v[i][j] = true;
        }
        for &(i, j) in d.iter() {
            if i > 0 && !v[i - 1][j] {
                p.push_back((i - 1, j));
            }
            if i < h - 1 && !v[i + 1][j] {
                p.push_back((i + 1, j));
            }
            if j > 0 && !v[i][j - 1] {
                p.push_back((i, j - 1));
            }
            if j < w - 1 && !v[i][j + 1] {
                p.push_back((i, j + 1));
            }
        }
        q = p;
    }
    println!("{}", v.iter().flatten().filter(|&&v| v).count());
}
