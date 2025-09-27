use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize,
        mut a: [u64; n]
    }
    a.append(& mut a.clone());
    let mut h = vec![0; n * 2 + 1];
    for i in 0..n * 2 {
        h[i + 1] = h[i] + a[i];
    }
    let mut s = 0;
    for _ in 0..q {
        I! {t: u8}
        if t == 1 {
            I! {c: usize}
            s += c;
            if s >= n {
                s -= n;
            }
        } else {
            I! {
                l: usize,
                r: usize
            }
            println!("{}", h[s + r] - h[s + l - 1]);
        }
    }
}
