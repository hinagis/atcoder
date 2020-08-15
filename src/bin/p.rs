use proconio::{input, marker::Usize1};

const M: u64 = 1000_000_007;

fn main() {
    input! {
        n: usize,
        xy: [(Usize1, Usize1); n - 1],
    }

    println!("{}", DP::new(n, &xy).calc(0, 0));
}

struct DP {
    dp: Vec<Option<(u64,u64)>>,
    t: Vec<Vec<usize>>,
}

impl DP {
    fn new(n: usize, xy: &Vec<(usize, usize)>) -> DP {
        let mut t = vec![vec![]; n];
        for &(x, y) in xy {
            t[x].push(y);
            t[y].push(x);
        }

        DP {
            dp: vec![None; n],
            t: t,
        }
    }

    fn calc(&mut self, i: usize, p: usize) -> u64 {
        let v = if let Some(v) = self.dp[i] {
            v
        } else {
            let mut v = (1, 1);
            for j in self.t[i].clone() {
                if j != p {
                    v.0 = v.0 * self.calc(j, i) % M;
                    v.1 = v.1 * self.dp[j].unwrap().0 % M;
                }
            }
            self.dp[i] = Some(v);
            v
        };
        (v.0 + v.1) % M
    }
}

