use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        q: u32,
    }
    let mut v = vec![1; n];
    let mut s = 0;
    for _ in 0..q {
        I! {
            x: U,
            y: U
        }
        let mut c = 0;
        for i in s..=x {
            c += v[i];
        }
        s = s.max(x + 1);
        v[y] += c;
        println!("{}", c);
    }
}
