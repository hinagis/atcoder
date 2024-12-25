use proconio::{input, marker::Usize1};

const M: u64 = 1000_000_007;

fn main() {
    input! {
        n: usize,
        xy: [(Usize1, Usize1); n - 1],
    }

    let mut t = vec![vec![]; n];
    for (x, y) in xy {
        t[x].push(y);
        t[y].push(x);
    }

    println!("{}", DP::new(n).calc(&t, 0, 0));
}

struct DP {
    dp: Vec<Option<(u64,u64)>>,
}

impl DP {
    fn new(n: usize) -> DP {
        DP {
            dp: vec![None; n],
        }
    }

    fn calc(&mut self, t: &Vec<Vec<usize>>, i: usize, p: usize) -> u64 {
        let v = if let Some(v) = self.dp[i] {
            v
        } else {
            let mut v = (1, 1);
            for &j in t[i].iter() {
                if j != p {
                    v.0 = v.0 * self.calc(t, j, i) % M;
                    v.1 = v.1 * self.dp[j].unwrap().0 % M;
                }
            }
            self.dp[i] = Some(v);
            v
        };
        (v.0 + v.1) % M
    }
}

