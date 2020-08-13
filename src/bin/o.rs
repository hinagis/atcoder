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

    let mut dp = vec![vec![None; 1 << n]; n + 1];
    println!("{}", calc(0, n, 0, &a, &mut dp));
}

fn calc(i: usize, n: usize, p: usize, a: &Vec<usize>, dp: &mut Vec<Vec<Option<u64>>>) -> u64 {
    if let Some(v) = dp[i][p] {
        v
    } else {
        let v = if i == n {
            1
        } else {
            let rest = a[i] & !p;
            let mut v = 0;
            for j in 0..n {
                if (rest >> j) & 1 == 1 {
                    v = (v + calc(i + 1, n, p | (1 << j), a, dp)) % M;
                }
            }
            v
        };
        dp[i][p] = Some(v);
        v
    }
}
