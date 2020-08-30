use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        ab: [(Usize1, Usize1); h],
    }

    let mut dp = vec![i32::max_value(); w];
    for j in 0..w {
        dp[j] = 0;
    }
    for i in 1..=h {
        let mut v = if dp[0] < i32::max_value() && ab[i - 1].0 > 0  {
            dp[0] + 1
        } else {
            i32::max_value()
        };
        dp[0] = v;
        let mut min = v;
        for j in 1..w {
            if v < i32::max_value() {
                v += 1;
            }
            if dp[j] < i32::max_value() && j < ab[i - 1].0 || j > ab[i - 1].1 {
                v = v.min(dp[j] + 1);
            }
            dp[j] = v;
            min = min.min(v);
        }
        println!("{}", if min == i32::max_value() {
            -1
        } else {
            min
        });
    }
}
