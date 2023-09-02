use proconio::input as I;

fn main() {
    I! {n: usize}

    let mut d = vec![vec![0; n]; n];
    for i in 0..n {
        for j in (i + 1)..n {
            I! {t: u64}
            d[i][j] = t;
        }
    }

    let bn = 1 << n;
    let mut dp = vec![0; bn];
    for b in 0..(1 << n) {
        let mut i = 0;
        while i < n && (b >> i) & 1 == 1 {
            i += 1;
        }
        if i == n {continue}

        dp[b | (1 << i)] = dp[b | (1 << i)].max(dp[b]);

        for j in (i + 1)..n {
            if (b >> j) & 1 == 1 {continue}

            let p = b | (1 << i) | (1 << j);
            dp[p] = dp[p].max(dp[b] + d[i][j]);
        }
    }

    println!("{}", dp[bn - 1]);
}
