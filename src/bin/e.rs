use proconio::{input, marker::Usize1};

fn main() {
    input! {
        r: usize,
        c: usize,
        k: usize,
        rcv: [(Usize1, Usize1, u64); k],
    }

    let b = make_board(&rcv, r, c);

    let mut dp = vec![vec![vec![None; 3 + 1]; c]; r];
    dp[0][0][0] = Some(0);
    for i in 0..r {
        for j in 0..c {
            for k in (0..3).rev() {
                if let Some(v) = dp[i][j][k] {
                    set_v(&mut dp, i, j, k + 1, v + b[i][j])
                }
            }
            for k in 0..=3 {
                if let Some(v) = dp[i][j][k] {
                    if i + 1 < r {
                        set_v(&mut dp, i + 1, j, 0, v)
                    }
                    if j + 1 < c {
                        set_v(&mut dp, i, j + 1, k, v)
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for k in 0..=3 {
        if let Some(v) = dp[r - 1][c - 1][k] {
            ans = ans.max(v);
        }
    }

    println!("{}", ans);
}

fn set_v(dp: &mut Vec<Vec<Vec<Option<u64>>>>, i: usize, j: usize, k: usize, v: u64) {
    let nv = &mut dp[i][j][k];
    let v = if let Some(nv) = *nv { nv.max(v) } else { v };
    *nv = Some(v);
}

fn make_board(rcv: &Vec<(usize, usize, u64)>, r: usize, c: usize) -> Vec<Vec<u64>> {
    let mut b = vec![vec![0; c]; r];
    for &(i, j, v) in rcv {
        b[i][j] = v;
    }
    b
}
