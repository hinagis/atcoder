use proconio::{input, marker::Usize1};

fn main() {
    input! {
        r: usize,
        c: usize,
        k: usize,
        rcv: [(Usize1, Usize1, u64); k],
    }

    let b = make_board(&rcv, r, c);

    let mut dp = vec![vec![0; c]; r];
    for j in 0..c {
        dp[0][j] = b[0][j];
    }

    for i in 1..r {
        for j in (0..c).rev() {
            let v = calc_max(&dp, &b, i - 1, j);
            dp[i - 1][j] = v;
            dp[i][j] = b[i][j] + v;
        }
    }

    println!("{}", calc_max(&dp, &b, r - 1, c - 1));
}

fn calc_max(dp: &Vec<Vec<u64>>, b: &Vec<Vec<u64>>, i: usize, j: usize) -> u64 {
    let mut v = dp[i][j];
    if j >= 1 {
        v = v.max(b[i][j] + dp[i][j - 1]);
        if j >= 2 {
            let mut m = b[i][j - 1];
            for k in (0..=(j - 2)).rev() {
                v = v.max(b[i][j] + m + dp[i][k]);
                m = m.max(b[i][k]);
            }
        }
    }
    v
}

fn make_board(rcv: &Vec<(usize, usize, u64)>, r: usize, c: usize) -> Vec<Vec<u64>> {
    let mut b = vec![vec![0; c]; r];
    for &(i, j, v) in rcv {
        b[i][j] = v;
    }
    b
}
