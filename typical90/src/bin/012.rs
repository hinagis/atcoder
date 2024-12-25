use proconio::{input, fastout, marker::Usize1};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }
    let mut f = vec![vec![false; w]; h];
    let mut ut = UT::new(h, w);
    for _ in 0..q {
        input!{
            t: u8,
            a: (Usize1, Usize1)
        }
        let (r, c) = a;
        if t == 1 {
            f[r][c] = true;
            if r > 0 && f[r - 1][c] {ut.unite(a, (r - 1, c))}
            if c > 0 && f[r][c - 1] {ut.unite(a, (r, c - 1))}
            if r < h - 1 && f[r + 1][c] {ut.unite(a, (r + 1, c))}
            if c < w - 1 && f[r][c + 1] {ut.unite(a, (r, c + 1))}
        } else {
            input!{b: (Usize1, Usize1)}
            println!("{}", if f[r][c] && f[b.0][b.1] && ut.root(a) == ut.root(b) {"Yes"} else {"No"});
        }
    }
}

struct UT {
    h: usize,
    w: usize,
    p: Vec<Vec<(usize, usize)>>
}

impl UT {
    pub fn new(h: usize, w: usize) -> Self {
        Self {
            h,
            w,
            p: vec![vec![(h, w); w]; h]
        }
    }

    pub fn root(self: &Self, p: (usize, usize)) -> (usize, usize) {
        let (r, c) = p;
        if self.p[r][c] == (self.h, self.w) {
            p
        } else {
            self.root(self.p[r][c])
        }
    }

    pub fn unite(self: &mut Self, u: (usize, usize), v: (usize, usize)) {
        let (u, v) = (self.root(u), self.root(v));
        if u != v {
            self.p[u.0][u.1] = v;
        }
    }
}
