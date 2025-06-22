use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize
    }
    let mut a = (1..=n).collect::<Vec<_>>();
    let mut s = 0;
    for _ in 0..q {
        I! {t: u8}
        match t {
            1 => {
                I! {
                    p: U,
                    x: usize
                }
                a[(s + p) % n] = x;
            },
            2 => {
                I! {p: U}
                println!("{}", a[(s + p) % n]);
            },
            _ => {
                I! {k: usize}
                s = (s + k) % n;
            }
        }
    }
}
