use proconio::input as I;

const M: i64 = 998244353;

fn main() {
    I! {
        n: usize,
        m: usize,
        b: usize,
        w: usize
    }

    let mut f = vec![(1, 1); 2501];
    for i in 2..2501 {
        f[i].0 = f[i - 1].0 * i as i64 % M;
        f[i].1 = mod_inv(f[i].0 as i64);
    }

    let g = |a: usize, b: usize| {
        if a < b {
            0
        } else {
            f[a].0 * f[a - b].1 % M * f[b].1 % M
        }
    };

    let mut dp = vec![vec![(0, 0); m + 1]; n + 1];
    for n in 1..=n {
        for m in 1..=m {
            let mut t = (0, 0);
            for i in 1..=n {
                for j in 1..=m {
                    let g = g(n, i) * g(m, j) % M; 
                    t.0 += g * dp[i][j].0;
                    t.0 %= M;
                    t.1 += g * dp[i][j].1;
                    t.1 %= M;
                }
            }
            dp[n][m] = (
                (g(n * m, b) + M - t.0) % M,
                (g(n * m, w) + M - t.1) % M
            );
        }
    }

    let mut r = 0;
    for bx in 1..=n {
        for wx in 1..=n - bx {
            for by in 1..=m {
                for wy in 1..=m - by {
                    r += dp[wx][wy].0 * dp[bx][by].1 % M
                        * g(n, wx) % M
                        * g(n - wx, bx) % M
                        * g(m, wy) % M
                        * g(m - wy, by) % M;
                    r %= M;
                }
            }
        }
    }
    println!("{}", r);
}

fn mod_pow(a: i64, b: i64) -> i64 {
    if b == 0 {
        1
    } else {
        if b % 2 == 0 {
            mod_pow(a * a % M, b / 2)
        } else {
            a * mod_pow(a, b - 1) % M
        }
    }
}

fn mod_inv(a: i64) -> i64 {
    mod_pow(a, M - 2) % M
}
