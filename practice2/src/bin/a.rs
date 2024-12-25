fn main() {
    proconio::input! {
        n: usize,
        q: usize,
        tuv: [(usize, usize, usize); q],
    }

    let mut dsu = DSU::new(n);

    for i in 0..q {
        let (t, u, v) = tuv[i];
        if t == 1 {
            println!("{}", if dsu.same(u, v) { 1 } else { 0 });
        } else {
            dsu.merge(u, v)
        }
    }
}

struct DSU {
    n: usize,
    parent: Vec<Option<usize>>,
    rank: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            n,
            parent: vec![None; n],
            rank: vec![1; n],
        }
    }

    fn merge(&mut self, u: usize, v: usize) {
        assert!(u < self.n);
        assert!(v < self.n);

        let (up, vp) = (self.top(u), self.top(v));
        if up != vp {
            let (p, c) = if self.rank[up] > self.rank[vp] { (up, vp) } else { (vp, up) };
            self.rank[p] += self.rank[c];
            self.rank[c] = 0;
            self.parent[c] = Some(p);
        }
    }

    fn same(&mut self, u: usize, v: usize) -> bool {
        assert!(u < self.n);
        assert!(v < self.n);

        self.top(u) == self.top(v)
    }

    fn top(&mut self, u: usize) -> usize {
        assert!(u < self.n);

        if let Some(p) = self.parent[u] {
            let p = self.top(p);
            self.parent[u] = Some(p);
            p
        } else {
            u
        }
    }
}
