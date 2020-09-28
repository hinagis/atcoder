use proconio::input;

const M: usize = 300000;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut st = SegTree::new(M, std::cmp::max);
    for _ in 0..n {
        input! {a: usize}
        let l = if a > k {a - k} else {0};
        let r = (a + k).min(M);
        let m = st.get(l, r + 1, None) + 1;
        st.set(a, m);
    }

    println!("{}", st.get(0, M + 1, None));
}


struct SegTree {
    tree: Vec<u64>,
    leaf_size: usize,
    op: fn(u64, u64) -> u64,
}

impl SegTree {
    fn new(n: usize, op: fn(u64, u64) -> u64) -> Self {
        let mut ls = 1;
        while ls < n {
            ls <<= 1;
        }
        Self {
            tree: vec![0; 2 * ls - 1],
            leaf_size: ls,
            op,
        }
    }

    fn set(&mut self, mut i: usize, v: u64) {
        i += self.leaf_size - 1;
        self.tree[i] = v;
        while i > 0 {
            i = (i - 1) / 2;
            self.tree[i] = (self.op)(self.tree[2 * i + 1], self.tree[2 * i + 2]);
        }
    }

    fn get(&self, s: usize, e: usize, rng: Option<(usize, usize, usize)>) -> u64 {
        let (i, l, r) = if let Some(rng) = rng { rng } else { (0, 0, self.leaf_size) };
        if l >= e || r <= s {
            0
        } else if l >= s && r <= e {
            self.tree[i]
        } else {
            let m = (l + r) / 2;
            let vl = self.get(s, e, Some((2 * i + 1, l, m)));
            let vr = self.get(s, e, Some((2 * i + 2, m, r)));
            (self.op)(vl, vr)
        }
    }
}
