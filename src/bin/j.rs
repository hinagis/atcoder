use proconio::{input, fastout, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut st = SegTree::new(n, std::cmp::max);
    for i in 0..n {
        input! {a: u64}
        st.set(i, a);
    }
    for _ in 0..q {
        input! { t: u8 }
        match t {
            1 => {
                input! {x: Usize1, v: u64}
                st.set(x, v);
            },
            2 => {
                input! {l: Usize1, r: Usize1}
                println!("{}", st.get(l, r + 1, None));
            },
            3 => {
                input! {x: Usize1, v: u64}
                let r = if st.get(x, x + 1, None) >= v {
                    x + 1
                } else if st.get(x, n, None) < v {
                    n + 1
                } else {
                    let mut l = x + 1;
                    let mut r = n;
                    while r - l > 1 {
                        let mid = (l + r) / 2;
                        if st.get(x, mid, None) < v {
                            l = mid;
                        } else {
                            r = mid;
                        }
                    }
                    r
                };
                println!("{}", r);
            },
            _ => (),
        }
    }
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
