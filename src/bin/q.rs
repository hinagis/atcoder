use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        h: [Usize1; n],
        a: [u64; n],
    }

    let mut dp = vec![0; n];
    for j in h[0]..n {
        dp[j] = a[0];
    }
    for i in 1..n {
        dp[h[i]] += a[i];
        for j in (h[i] + 1)..n {
            if dp[j] < dp[h[i]] {
                dp[j] = dp[h[i]]
            }
        }
    }

    let mut m = 0;
    for j in 0..n {
        if dp[j] > m {
            m = dp[j]
        }
    }
    println!("{}", m);
}
