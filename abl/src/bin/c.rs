use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut dsu = DSU::new(n + 1);
    for _ in 0..m {
        input!{a: Usize1, b: Usize1}
        dsu.merge(a, b);
    }
    let mut h = std::collections::HashSet::new();
    for i in 0..n {
        h.insert(dsu.top(i));
    }
    println!("{}", h.len() - 1);
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

    #[allow(unused)]
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
