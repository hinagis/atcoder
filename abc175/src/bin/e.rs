use proconio::{input, marker::Usize1};

fn main() {
    input! {
        r: usize,
        c: usize,
        k: usize,
        rcv: [(Usize1, Usize1, u64); k],
    }

    println!("{}", DP::new(&rcv, r, c).calc());
}

struct DP {
    dp: Vec<Vec<Vec<Option<u64>>>>,
    b: Vec<Vec<u64>>,
    r: usize,
    c: usize,
}

impl DP {
    fn new(rcv: &Vec<(usize, usize, u64)>, r: usize, c: usize) -> DP {
        DP {
            dp: vec![vec![vec![None; 3 + 1]; c]; r],
            b: DP::make_board(&rcv, r, c),
            r: r,
            c: c,
        }
    }

    fn make_board(rcv: &Vec<(usize, usize, u64)>, r: usize, c: usize) -> Vec<Vec<u64>> {
        let mut b = vec![vec![0; c]; r];
        for &(i, j, v) in rcv {
            b[i][j] = v;
        }
        b
    }

    fn calc(&mut self) -> u64 {
        let (r, c) = (self.r, self.c);
        self.dp[0][0][0] = Some(0);
        for i in 0..r {
            for j in 0..c {
                for k in (0..3).rev() {
                    if let Some(v) = self.dp[i][j][k] {
                        self.set_v(i, j, k + 1, v + self.b[i][j]);
                    }
                }
                for k in 0..=3 {
                    if let Some(v) = self.dp[i][j][k] {
                        if i + 1 < r {
                            self.set_v(i + 1, j, 0, v);
                        }
                        if j + 1 < c {
                            self.set_v(i, j + 1, k, v);
                        }
                    }
                }
            }
        }
    
        self.get_ans()
    }

    fn set_v(&mut self, i: usize, j: usize, k: usize, v: u64) {
        let nv = &mut self.dp[i][j][k];
        let v = if let Some(nv) = *nv { nv.max(v) } else { v };
        *nv = Some(v);
    }

    fn get_ans(&self) -> u64 {
        let mut ans = 0;
        for k in 0..=3 {
            if let Some(v) = self.dp[self.r - 1][self.c - 1][k] {
                ans = ans.max(v);
            }
        }
        ans
    }
}
