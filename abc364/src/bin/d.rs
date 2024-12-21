use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize, q: usize,
        mut a: [i64; n],
    }
    for _ in 0..q {
        I! {
            b: i64,
            k: usize
        }
        a.sort_by(|&u, &v| (u - b).abs().cmp(&(v - b).abs()));
        println!("{}", (a[k - 1] - b).abs());
    }
}
