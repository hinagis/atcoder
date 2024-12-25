use proconio::input;

const M: u64 = 1000_000_007;

fn calc(n: usize, k: usize, a: &Vec<usize>) -> u64 {
    let mut dp = vec![vec![0; k + 1]; n];
    for j in 0..=a[0] {
        dp[0][j] = 1;
    }
    for j in (a[0] + 1)..=k {
        dp[0][j] = 0;
    }
    for i in 1..n {
        let mut cum = vec![0; k + 2];
        for j in 1..=(k + 1) {
            cum[j] = (cum[j - 1] + dp[i - 1][j - 1]) % M;
        }
        for j in 0..=k {
            let ks = if a[i] > j { 0 } else {j - a[i]};
            dp[i][j] = (cum[j + 1] + M - cum[ks]) % M;
        }
    }
    dp[n - 1][k]
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    println!("{}", calc(n, k, &a));
}
