use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        h: usize,
        w: usize,
        x: u64,
        p: U,
        q: U,
        s: [[u64; w]; h],
    }
    let mut b = s[p][q];
    let mut t = std::collections::HashSet::new();
    let mut f = vec![vec![true; w]; h];
    f[p][q] = false;
    if p > 0     {t.insert((p - 1, q));}
    if q > 0     {t.insert((p, q - 1));}
    if p < h - 1 {t.insert((p + 1, q));}
    if q < w - 1 {t.insert((p, q + 1));}
    let mut e = true;
    while e {
        e = false;
        let mut a = vec![];
        let mut d = vec![];
        for &(i, j) in t.iter() {
            if s[i][j] * x < b {
                e = true;
                b += s[i][j];
                f[i][j] = false;
                d.push((i, j));
                if i > 0     && f[i - 1][j] {a.push((i - 1, j));}
                if j > 0     && f[i][j - 1] {a.push((i, j - 1));}
                if i < h - 1 && f[i + 1][j] {a.push((i + 1, j));}
                if j < w - 1 && f[i][j + 1] {a.push((i, j + 1));}
            }
        }
        for a in a {
            t.insert(a);
        }
        for d in d {
            t.remove(&d);
        }
    }
    println!("{}", b);
}
