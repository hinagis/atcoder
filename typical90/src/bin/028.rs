
use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [(usize, usize, usize, usize); n]
    }

    let mut f = vec![vec![0isize; 1002]; 1002];
    for &(l, t, r, b) in &p {
        f[t][l] += 1;
        f[b][r] += 1;
        f[t][r] -= 1;
        f[b][l] -= 1;
    }

    for i in 0..1001 {
        for j in 0..1001 {
            f[i][j + 1] += f[i][j]
        }
    }
    let mut c = vec![0; n + 1];
    for j in 0..1001 {
        for i in 0..1001 {
            c[f[i][j] as usize] += 1;
            f[i + 1][j] += f[i][j];
        }
    }

    for k in 1..=n {
        println!("{}", c[k]);
    }
}
