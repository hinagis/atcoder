use proconio::input;

struct DP {
    dp: Vec<Vec<Option<i64>>>,
    a: Vec<i64>,
}

impl DP {
    fn new(n: usize, a: Vec<i64>) -> DP {
        DP {
            dp: vec![vec![None; n]; n],
            a: a,
        }
    }

    fn calc(self: &mut DP, l: usize, r: usize) -> i64 {
        if let Some(v) = self.dp[l][r] {
            v
        } else {
            let v = if l == r {
                self.a[l]
            } else {
                std::cmp::max(
                    self.a[l] - self.calc(l + 1, r),
                    self.a[r] - self.calc(l, r - 1)
                )
            };
            self.dp[l][r] = Some(v);
            v
        }
    }
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    println!("{}", DP::new(n, a).calc(0, n - 1));
}
