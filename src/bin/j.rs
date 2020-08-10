use proconio::input;

fn calc(dp: &mut Vec<Vec<Vec<f64>>>, n: usize, i: usize, j: usize, k: usize) -> f64 {
    if dp[i][j][k] > 0.0 {
        dp[i][j][k]
    } else if i == 0 && j == 0 && k == 0 {
        0.0
    } else {
        let r = n as f64
            + if i > 0 { calc(dp, n, i - 1, j, k) * (i as f64) } else { 0.0 }
            + if j > 0 { calc(dp, n, i + 1, j - 1, k) * (j as f64) } else { 0.0 }
            + if k > 0 { calc(dp, n, i, j + 1, k - 1) * (k as f64) } else { 0.0 };
        dp[i][j][k] = r / ((i + j + k) as f64);
        dp[i][j][k]
    }
}

fn main() {
    input! {
        n: usize,
    }
    let mut ijk = [0; 3];
    for _ in 0..n {
        input! {
            a: usize,
        }
        ijk[a - 1] += 1;
    }
    let mut dp = vec![vec![vec![-1.0; n + 1]; n + 1]; n + 1];

    println!("{}", calc(&mut dp, n, ijk[0], ijk[1], ijk[2]));
}
