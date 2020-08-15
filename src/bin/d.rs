use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: Usize1,
        p: [Usize1; n],
        c: [i64; n],
    }

    let mut dp = DP::new(n, k);
    let mut r = dp.calc(0, k, &p, &c);
    for i in 1..n {
        r = r.max(dp.calc(i, k, &p, &c))
    }
    println!("{}", r);
}

struct DP {
    dp: Vec<Vec<Option<i64>>>,
}

impl DP {
        fn new(n: usize, k: usize) -> DP {
        DP {
            dp: vec![vec![None; k + 1]; n],
        }
    }
    fn calc(&mut self, i: usize, k: usize, p: &Vec<usize>, c: &Vec<i64>) -> i64 {
        let v = if let Some(v) = self.dp[i][k] {
            v
        } else {
            let v = c[p[i]];
            if k > 0 {
                v.max(v + self.calc(p[i], k - 1, p, c))
            } else {
                v
            }
        };
        self.dp[i][k] = Some(v);
        v
    }
}
