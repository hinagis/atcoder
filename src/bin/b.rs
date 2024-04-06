use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        p: [(i64, i64); n]
    }

    for i in 0..n {
        let (x, y) = p[i];
        let (mut d, mut m) = (0, 0);
        for j in 0..n {
            let (u, v) = p[j];
            let e = (x - u).pow(2) + (y - v).pow(2);
            if e > d {
                (d, m) = (e, j + 1);
            }
        }
        println!("{}", m);
    }
}
