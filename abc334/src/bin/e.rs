use proconio::{input as I, marker::Chars as C};
use std::collections::HashSet as H;

const M: usize = 998244353;

fn main() {
    I! {
        h: usize,
        w: usize,
        s: [C; h]
    }

    let mut u = petgraph::unionfind::UnionFind::new(h * w);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '#' {continue}

            if i > 0 && s[i - 1][j] == '#' {
                u.union(i * w + j, (i - 1) * w + j);
            }
            if j > 0 && s[i][j - 1] == '#' {
                u.union(i * w + j, i * w + (j - 1));
            }
        }
    }

    let mut t = H::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                t.insert(u.find(i * w + j));
            }
        }
    }

    let mut c = 0;
    let mut r = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {continue}

            r += 1;

            let mut g = H::new();
            if i > 0 && s[i - 1][j] == '#' {
                g.insert(u.find((i - 1) * w + j));
            }
            if i < h - 1 && s[i + 1][j] == '#' {
                g.insert(u.find((i + 1) * w + j));
            }
            if j > 0 && s[i][j - 1] == '#' {
                g.insert(u.find(i * w + (j - 1)));
            }
            if j < w - 1 && s[i][j + 1] == '#' {
                g.insert(u.find(i * w + (j + 1)));
            }
            c += t.len() + 1 - g.len();
            c %= M;
        }
    }
    println!("{}", (c * inv(r)) % M);
}

pub fn inv(mut v: usize) -> usize {
    let mut r: usize = 1;
    let mut n = M - 2;
    while n > 0 {
        if n % 2 == 1 {
            r = (r * v) % M;
        }
        v = (v * v) % M;
        n /= 2;
    }
    r
}
