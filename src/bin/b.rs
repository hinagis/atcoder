use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        m: usize,
        a: [[U; n]; m]
    }
    let mut f = vec![vec![false; n]; n];
    for i in 0..m {
        for j in 1..n {
            f[a[i][j]][a[i][j - 1]] = true;
            f[a[i][j - 1]][a[i][j]] = true;
        }
    }
    println!("{}", f.iter().fold(0, |s, f| s + f.iter().filter(|&&e| ! e).count() - 1) / 2);
}
