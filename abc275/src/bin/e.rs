const M: usize = 998_244_353;

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let p = m_inv(m);
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for _ in 0..k {
        let mut ndp = vec![0; n + 1];
        ndp[n] = dp[n];
        for i in 0..n {
            if dp[i] == 0 {
                continue;
            }
            for j in 1..=m {
                let ni = {let ni = i + j; if ni <= n {ni} else{n - (ni - n)}};
                ndp[ni] += dp[i] * p;
                ndp[ni] %= M;
            }
        }
        dp = ndp;
    }

    println!("{}", dp[n]);
}

fn m_exp(mut x: usize, mut y: usize) -> usize {
    let mut ans = 1;
    while y > 0 {
        if (y & 1) > 0 {
            ans = ans * x % M;
        }
        x = x * x % M;
        y >>= 1;
    }
    ans
}

fn m_inv(x: usize) -> usize {
    m_exp(x, M - 2)
}
