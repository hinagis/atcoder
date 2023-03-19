use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize
    }

    let mut v = vec![false; n];
    let mut c = 0;
    let mut s = 0;
    for _ in 0..q {
        I! {e: u8}
        if e == 1 {
            c += 1;
        } else if e == 2 {
            I! {x: usize}
            v[x - 1] = true;
            while s < c && v[s] {
                s += 1;
            }
        } else {
            println!("{}", s + 1);
        }
    }
}
