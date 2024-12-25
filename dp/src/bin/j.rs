use proconio::input;

struct DP {
    dp: Vec<Vec<Vec<f64>>>,
    n: usize,
}

impl DP {
    pub fn new(n: usize) -> DP {
        DP {
            dp: vec![vec![vec![-1.0; n + 1]; n + 1]; n + 1],
            n: n,
        }
    }

    fn calc(&mut self, i: usize, j: usize, k: usize) -> f64 {
        if self.dp[i][j][k] > 0.0 {
            self.dp[i][j][k]
        } else if i == 0 && j == 0 && k == 0 {
            0.0
        } else {
            let r = (self.n as f64
                + if i > 0 { self.calc(i - 1, j, k) * (i as f64) } else { 0.0 }
                + if j > 0 { self.calc(i + 1, j - 1, k) * (j as f64) } else { 0.0 }
                + if k > 0 { self.calc(i, j + 1, k - 1) * (k as f64) } else { 0.0 }
            ) / ((i + j + k) as f64);
            self.dp[i][j][k] = r;
            r
        }
    }
}

fn main() {
    input! {
        n: usize,
    }
    let mut ijk = [0; 3];
    for _ in 0..n {
        input! {
            a: usize,
        }
        ijk[a - 1] += 1;
    }

    println!("{}", DP::new(n).calc(ijk[0], ijk[1], ijk[2]));
}
