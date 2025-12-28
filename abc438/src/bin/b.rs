use proconio::{input as I, marker::Bytes as B};

fn main() {
    I! {
        n: usize,
        m: usize,
        s: B,
        t: B
    }
    let mut k = u32::MAX;
    for i in 0..=n - m {
        let mut c = 0;
        for j in 0..m {
            c += (s[i + j] + if s[i + j] >= t[j] {0} else {10} - t[j]) as u32;
        }
        k = k.min(c);
    }
    println!("{}", k);
}
