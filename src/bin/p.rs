use std::collections::HashMap;

const M: u64 = 1000_000_007;

fn main() {
    proconio::input! {
        n: usize,
        xy: [(usize, usize); n - 1],
    }

    println!("{}", DP::new(n, &xy).calc(1));
}

struct DP {
    dp: Vec<Option<(u64,u64)>>,
    t: HashMap<usize, Vec<usize>>,
}

impl DP {
    fn new(n: usize, xy: &Vec<(usize, usize)>) -> DP {
        let mut g = HashMap::new();
        for (x, y) in xy {
            g.entry(*x).or_insert(Vec::new()).push(*y);
            g.entry(*y).or_insert(Vec::new()).push(*x);
        }

        let mut t = HashMap::new();
        if let Some(c) = g.get_mut(&1) {
            let mut p = vec![(1, t.entry(1).or_insert(c.clone()).clone())];
            while p.is_empty() == false {
                for i in 0..p[0].1.len() {
                    if let Some(c) = g.get_mut(&p[0].1[i]) {
                        c.retain(|ci| *ci != p[0].0);
                        p.push((p[0].1[i], t.entry(p[0].1[i]).or_insert(c.clone()).clone()));
                    }
                }
                p.remove(0);
            }
        }

        DP {
            dp: vec![None; n],
            t: t,
        }
    }

    fn calc(&mut self, i: usize) -> u64 {
        if let Some(v) = self.dp[i - 1] {
            (v.0 + v.1) % M
        } else {
            let mut v = (1, 1);
            if self.t.contains_key(&i) {
                let clen = { self.t.get(&i).unwrap().len() };
                for j in 0..clen {
                    let j = { self.t.get(&i).unwrap()[j] };
                    v.0 = (v.0 * self.calc(j)) % M;
                    v.1 = (v.1 * self.dp[j - 1].unwrap().0) % M;
                }
            }
            self.dp[i - 1] = Some(v);
            (v.0 + v.1) % M
    }
    }
}

