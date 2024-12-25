use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize,
    }
    let mut v = vec![vec![]; n];
    for i in 0..n {
        I! {l: usize}
        v[i] = vec![0; l];
        for j in 0..l {
            I! {a: u32}
            v[i][j] = a;
        }
    }

    for _ in 0..q {
        I! {
            s: U,
            t: U
        }
        println!("{}", v[s][t]);
    }
}
