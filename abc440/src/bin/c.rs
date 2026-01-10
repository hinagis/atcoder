use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {t: u32}
    for _ in 0..t {
        I! {
            n: usize,
            w: usize,
            c: [u64; n]
        }
        let mut v = vec![0; 2 * w];
        for i in 0..n {
            v[i % (2 * w)] += c[i];
        }
        let mut c = 0;
        for i in 0..w {
            c += v[i];
        }
        let mut m = c;
        for i in 0..2 * w - 1 {
            c += v[(i + w) % (2 * w)];
            c -= v[i];
            m = m.min(c);
        }
        println!("{}", m);
    }
}
