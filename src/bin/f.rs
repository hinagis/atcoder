const M: usize = 998244353;

fn main() {
    proconio::input! {s: String}

    let s = s.bytes().map(|b| (b - b'a') as usize).collect::<Vec<_>>();
    let c = (0..26).map(|i| s.iter().filter(|&&j| j == i).count()).collect::<Vec<_>>();

    let n = s.len();
    let mut fr = vec![1; n + 1];
    for i in 2..=n {
        fr[i] = (fr[i - 1] * i) % M;
    }
    let mut ifr = vec![1; n + 1];
    ifr[n] = inv(fr[n]);
    for i in (2..n).rev() {
        ifr[i] = (ifr[i + 1] * (i + 1)) % M;
    }

    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    let mut t = 0;
    for i in 0..26 {
        if c[i] > 0 {
            for j in (0..=t).rev() {
                for k in (1..=c[i]).rev() {
                    let v = (dp[j] * fr[j + k]) % M;
                    let v = (v * ifr[j]) % M;
                    let v = (v * ifr[k]) % M;
                    dp[j + k] = (dp[j + k] + v) % M;
                }
            }

            t += c[i];
        }
    }

    let mut r = 0;
    for i in 1..=n {
        r = (r + dp[i]) % M;
    }

    println!("{}", r);
}

fn exp(mut x: usize, mut y: usize) -> usize {
    let mut r = 1;
    while y > 0 {
        if (y & 1) == 1 {
            r = r * x % M;
        }
        x = x * x % M;
        y >>= 1;
    }
    r
}

fn inv(x: usize) -> usize {
    exp(x, M - 2)
}
