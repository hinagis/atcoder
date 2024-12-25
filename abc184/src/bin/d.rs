fn main() {
    proconio::input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let mut dp = vec![vec![vec![None; 101]; 101]; 101];
    println!("{}", calc(&mut dp, a, b, c));
}

fn calc(dp: &mut Vec<Vec<Vec<Option<f64>>>>, i: usize, j: usize, k: usize) -> f64 {
    if i >= 100 || j >= 100 || k >= 100 {
        0.0f64
    } else {
        let fi = i as f64 * (if let Some(v) = dp[i + 1][j][k] {v} else {calc(dp, i + 1, j, k)} + 1.0f64);
        let fj = j as f64 * (if let Some(v) = dp[i][j + 1][k] {v} else {calc(dp, i, j + 1, k)} + 1.0f64);
        let fk = k as f64 * (if let Some(v) = dp[i][j][k + 1] {v} else {calc(dp, i, j, k + 1)} + 1.0f64);
        let v = (fi + fj + fk) / ((i + j + k) as f64);
        dp[i][j][k] = Some(v);
        v
    }
}
