use proconio::{input as I, fastout, marker::Bytes};

const M: u64 = 998244353;

#[fastout]
fn main() {
    I! {t: usize}
    for _ in 0..t {
        I! {n: usize, s: Bytes}
        let mut r = 0;
        let mut c = s.clone();
        let h = (n + 1) / 2;
        for i in 0..h {
            r = (r * 26 + (s[i] - b'A') as u64) % M;
            c[n - 1 - i] = s[i];
        }
        if c <= s {
            r = (r + 1) % M;
        }
        println!("{}", r);
    }
}
