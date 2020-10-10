use proconio::{input, marker::Chars};

const M: u64 = 1000_000_007;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut dp = vec![vec![0; w]; h];
    for i in 0..h {
        let mut upd = |l: usize, r: usize| {
            for j in l..r {
                dp[i][j] += r - l;
            }
        };
        let mut l = 0;
        let mut r = 0;
        while r < w {
            if s[i][r] == '#' {
                upd(l, r);
                l = r + 1;
            }
            r += 1;
        }
        upd(l, r);
    }
    for j in 0..w {
        let mut upd = |t: usize, b: usize| {
            for i in t..b {
                dp[i][j] += b - t;
            }
        };
        let mut t = 0;
        let mut b = 0;
        while b < h {
            if s[b][j] == '#' {
                upd(t, b);
                t = b + 1;
            }
            b += 1;
        }
        upd(t, b);
    }
    let k = count_period(&s);
    let mut sum = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                sum += modpow(2, k - (dp[i][j] as u64 - 1));
                sum %= M;
            }
        }
    }

    let mut r = modpow(2, k);
    r = (r * k) % M;
    r = (r + M - sum) % M;

    println!("{}", r);
}

fn count_period(s: &Vec<Vec<char>>) -> u64 {
    let mut k = 0;
    for i in 0..s.len() {
        for j in 0..s[0].len() {
            if s[i][j] == '.' {
                k += 1;
            }
        }
    }
    k
}

fn modpow(x: u64, y: u64) -> u64 {
    let mut result = 1;
    let mut a = x;
    let mut b = y;
    while b > 0 {
        if b % 2 == 1 {
            result = result * a % M;
        }
        a = a * a % M;
        b /= 2;
    }
    result
}
