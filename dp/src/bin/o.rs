use proconio::input;

const M: u64 = 1000_000_007;

fn main() {
    input! { n: usize }

    let mut a = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            input! { aij: usize };
            a[i] |= aij << j;
        }
    }

    println!("{}", DP::new(n, &a).calc(0));
}

struct DP<'a> {
    dp: Vec<Option<u64>>,
    b: usize,
    a: &'a Vec<usize>,
}

impl<'a> DP<'a> {
    fn new(n: usize, a: &'a Vec<usize>) -> DP<'a> {
        let mut b = 0;
        for i in 0..n {
            b |= 1 << i;
        }
        DP {
            dp: vec![None; 1 << n],
            b: b,
            a: a,
        }
    }

    fn calc(&mut self, p: usize) -> u64 {
        if let Some(v) = self.dp[p] {
            v
        } else {
            let v = if p == self.b {
                1
            } else {
                let rest = self.a[p.count_ones() as usize] & !p;
                let mut v = 0;
                let mut b = 1;
                while self.b & b != 0 {
                    if rest & b == b {
                        v = (v + self.calc(p | b)) % M;
                    }
                    b <<= 1;
                }
                v
            };
            self.dp[p] = Some(v);
            v
        }
    }
}
