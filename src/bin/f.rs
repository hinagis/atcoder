const C: usize = 42;

fn main() {
    proconio::input! {
        n: usize,
        a: [u64; n],
    }

    let s = a[0] + a[1];
    let mut x = 0;
    for i in 2..n {
        x ^= a[i];
    }

    let mut dp = vec![vec![vec![a[0] + 1; 2]; 2]; C];
    dp[0][0][0] = 0;
    for i in 0..(C - 1) {
        let cx = (x >> i) & 1;
        let cs = (s >> i) & 1;
        let ca = (a[0] >> i) & 1;

        for j in 0..2 {
            for k in 0..2 {
                if dp[i][j][k] > a[0] {
                    continue;
                }
                for na in 0..2 {
                    for nb in 0..2 {
                        if (na ^ nb) != cx {
                            continue;
                        }
                        let ns = na + nb + j as u64;
                        if ns % 2 != cs {
                            continue;
                        }
                        let ni = i + 1;
                        let nj = if ns >= 2 { 1 } else { 0 };
                        let nk = if ca < na { 1 } else if ca == na { k } else { 0 };
                        let v = dp[i][j][k] | ((1 << i as u64) * na);
                        if dp[ni][nj][nk] > a[0] || v > dp[ni][nj][nk] {
                            dp[ni][nj][nk] = v;
                        }
                    }
                }
            }
        }
    }

    let r = dp[C - 1][0][0];
    if r <= a[0] && r > 0 {
        println!("{}", a[0] - r);
    } else {
        println!("-1");
    }
}
