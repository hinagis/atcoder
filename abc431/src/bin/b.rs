use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        mut x: u32,
        n: usize,
        w: [u32; n],
        q: u32
    }
    let mut f = vec![false; n];
    for _ in 0..q {
        I! {p: U}
        if f[p] {
            x -= w[p];
        } else {
            x += w[p];
        }
        f[p] ^= true;
        println!("{}", x);
    }
}
