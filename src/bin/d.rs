use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut uvw: [(Usize1, Usize1, u64); n - 1]
    }
    uvw.sort_by(|(_, _, a), (_, _, b)| a.cmp(b));

    let mut t = UT::new(n);
    let mut r = 0;
    for &(u, v, w) in &uvw {
        r += w * t.size(u) as u64 * t.size(v) as u64;
        t.unite(u, v);
    }
    println!("{}", r);
}

pub struct UT {
    p: Vec<usize>,
    r: Vec<bool>
}

impl UT {
    pub fn new(n: usize) -> Self {
        Self {
            p: vec![1; n],
            r: vec![true; n]
        }
    }

    pub fn unite(&mut self, u: usize, v: usize) -> usize {
        let (r, c) = (self.root(u), self.root(v));
        if r == c {
            return r;
        }
        let (r, c) = if self.p[r] < self.p[c] {(c, r)} else {(r, c)};
        self.p[r] += self.p[c];
        self.p[c] = r;
        self.r[c] = false;
        r
    }

    pub fn root(&mut self, u: usize) -> usize {
        if self.r[u] {
            return u;
        }
        let r = self.root(self.p[u]);
        self.p[u] = r;
        r
    }

    pub fn size(&mut self, u: usize) -> usize {
        let r = self.root(u);
        self.p[r]
    }
}
