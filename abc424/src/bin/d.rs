use itertools::iproduct;
use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {t: u32}
    for _ in 0..t {
        I! {
            h: usize,
            w: usize,
            mut s: [C; h]
        }
        let mut a = vec![vec![true; 1 << w]; 1 << w];
        for (i, j) in iproduct!(0..1 << w, 0..1 << w) {
            a[i][j] &= !(0..w - 1)
                .any(|k| ((i >> k) & 3 == 3) & ((j >> k) & 3 == 3));
        }

        let mut dp = vec![50; 1 << w];
		dp[0] = 0;
        for i in 0..h {
			let b = (0..w)
                .filter(|&j| s[i][j] == '#')
                .fold(0, |b, j| b | 1 << j);
            let mut dp2 = vec![50; 1 << w];
            for j in 0..1 << w {
				if (j | b) == b {
                    for k in 0..1 << w {
						if a[k][j] {
                            dp2[j] = dp2[j].min(dp[k] + usize::count_ones(j ^ b));
                        }
					}
				}
			}
			dp = dp2;
		}
        println!("{}", dp.iter().fold(50, |a, &x| a.min(x)));
    }
}
