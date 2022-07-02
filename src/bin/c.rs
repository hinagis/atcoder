use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize,
        s: C,
    }
    let mut p = 0;
    for _ in 0..q {
        I! {
            q: u8,
            x: usize
        }
        if q == 1 {
            p = (p + n - x) % n;
        } else {
            println!("{}", s[(p + x - 1) % n]);
        }
    }
}
