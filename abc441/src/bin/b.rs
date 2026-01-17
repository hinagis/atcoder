use proconio::{input as I, fastout as F, marker::Bytes as B};

#[F]
fn main() {
    I! {
        _: u8,
        _: u8,
        s: B,
        t: B,
        q: usize,
    }
    let mut u = [false; 26];
    let mut v = [false; 26];
    for b in s {
        u[(b - b'a') as usize] = true;
    }
    for b in t {
        v[(b - b'a') as usize] = true;
    }
    for _ in 0..q {
        I! {w: B}
        let mut t = true;
        let mut a = true;
        for b in w {
            t &= u[(b - b'a') as usize];
            a &= v[(b - b'a') as usize];
        }
        println!("{}", if t && a {
            "Unknown"
        } else if t {
            "Takahashi"
        } else {
            "Aoki"
        });
    }
}
