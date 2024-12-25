use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        h: [(String, u32); n]
    }
    let mut s = 0;
    for i in 0..n {
        if h[i].1 < h[s].1 {
            s = i;
        }
    }
    for i in 0..n {
        println!("{}", h[(i + s) % n].0);
    }
}
