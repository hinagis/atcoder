use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        a: [[[u64; n]; n]; n],
        q: usize,
    }
    let mut s = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                s[i + 1][j + 1][k + 1] = s[i][j + 1][k + 1] + s[i + 1][j][k + 1] + s[i + 1][j + 1][k]
                                       + s[i][j][k] + a[i][j][k]
                                       - s[i][j][k + 1] - s[i][j + 1][k] - s[i + 1][j][k];
            }
        }
    }
    for _ in 0..q {
        I! {
            x: (U, U),
            y: (U, U),
            z: (U, U),
        }
        println!("{}", s[x.1 + 1][y.1 + 1][z.1 + 1]
            + s[x.0][y.0][z.1 + 1] + s[x.0][y.1 + 1][z.0] + s[x.1 + 1][y.0][z.0]
            - s[x.0][y.1 + 1][z.1 + 1] - s[x.1 + 1][y.0][z.1 + 1] - s[x.1 + 1][y.1 + 1][z.0]
            - s[x.0][y.0][z.0]
        );
    }
}
