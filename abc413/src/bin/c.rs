use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {q: u32}
    let mut a = std::collections::VecDeque::new();
    for _ in 0..q {
        I! {t: u8}
        if t == 1 {
            I! {
                c: u64,
                x: u64
            }
            a.push_back((c, x));
        } else {
            I! {mut k: u64}
            let mut s = 0;
            while k > 0 {
                let (c, x) = a.pop_front().unwrap();
                if c > k {
                    s += k * x;
                    a.push_front((c - k, x));
                    k = 0;
                } else {
                    s += c * x;
                    k -= c;
                }
            }
            println!("{}", s);
        }
    }
}
