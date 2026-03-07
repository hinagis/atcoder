use omniswap::swap;
use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize,
        mut a: [u32; n]
    }
    let mut c = vec![0; n + 1];
    for i in 0..n {
        c[i + 1] = c[i] + a[i];
    }
    for _ in 0..q {
        I! {k: u8}
        if k == 1 {
            I! {x: usize}
            c[x] += a[x];
            c[x] -= a[x - 1];
            swap!(&mut a[x - 1], &mut a[x]);
        } else {
            I! {
                l: usize,
                r: usize
            }
            println!("{}", c[r] - c[l - 1]);
        }
    }
}
