use proconio::{input, marker::Usize1};
fn main() {
    input! {
        h: usize,
        w: usize,
        m: usize,
        hw: [(Usize1, Usize1); m]
    }
    let mut sum_h = vec![0; h];
    let mut sum_v = vec![0; w];
    let mut bomb = vec![vec![false; w]; h];
    for (i, j) in hw {
        bomb[i][j] = true;
        sum_h[i] += 1;
        sum_v[j] += 1;
    }

    let mut r = 0;
    for i in 0..h {
        for j in 0..w {
            r = r.max(sum_h[i] + sum_v[j] - if bomb[i][j] { 1 } else { 0 });
        }
    }

    println!("{}", r);
}
