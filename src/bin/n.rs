fn main() {
    proconio::input! {
        n: usize,
        a: [u64; n],
    }

    println!("{}", DP::new(n, a).calc(0, n - 1));
}

struct DP {
    dp: Vec<Vec<Option<u64>>>,
    cum: Vec<u64>,
}

impl DP {
    fn new(n: usize, a: Vec<u64>) -> DP {
        let mut cum = vec![0; n + 1];
        for i in 1..=n {
            cum[i] = cum[i - 1] + a[i - 1];
        }
        DP {
            dp: vec![vec![None; n]; n],
            cum: cum,
        }
    }

    fn calc(&mut self, l: usize, r: usize) -> u64 {
        if let Some(v) = self.dp[l][r] {
            v
        } else {
            let v = if l == r {
                0
            } else {
                let mut v = std::u64::MAX;
                for m in l..r {
                    v = std::cmp::min(v, self.calc(l, m) + self.calc(m + 1, r))
                }
                v + self.cum[r + 1] - self.cum[l]
            };
            self.dp[l][r] = Some(v);
            v
        }
    }
}
