use proconio::{input, marker::Usize1};

struct SegmetTree {
    tree: Vec<u64>,
    leaf_size: usize,
}

impl SegmetTree {
    fn new(n: usize) -> SegmetTree {
        let mut ls = 1;
        while ls < n {
            ls <<= 1;
        }
        SegmetTree {
            tree: vec![0; 2 * ls - 1],
            leaf_size: ls,
        }
    }

    fn update_max(&mut self, i: usize, v: u64) {
        let mut i = i + self.leaf_size - 1;
        self.tree[i] = v;
        while i > 0 {
            i = (i - 1) / 2;
            self.tree[i] = self.tree[2 * i + 1].max(self.tree[2 * i + 2]);
        }
    }

    fn get_max(&self, e: usize, rng: Option<(usize, usize, usize)>) -> u64 {
        let (i, l, r) = if let Some(rng) = rng { rng } else { (0, 0, self.leaf_size) };
        if l >= e || r <= 0 {
            0
        } else if r <= e {
            self.tree[i]
        } else {
            let m = (l + r) / 2;
            let vl = self.get_max(e, Some((2 * i + 1, l, m)));
            let vr = self.get_max(e, Some((2 * i + 2, m, r)));
            vl.max(vr)
        }
    }
}

fn main() {
    input! {
        n: usize,
        h: [Usize1; n],
        a: [u64; n],
    }

    let mut dp = SegmetTree::new(n);
    dp.update_max(h[0], a[0]);
    for i in 1..n {
        let m = dp.get_max(h[i] + 1, None);
        dp.update_max(h[i], m + a[i]);
    }

    println!("{}", dp.get_max(n, None));
}
