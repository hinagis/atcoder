use proconio::{input as I, marker::Bytes as B};

fn main() {
    I! {s: B}
    let mut r = 0u64;
    let mut f = false;
    let mut c = vec![0; 26];
    for &b in s.iter().rev() {
        let b = (b - b'a') as usize;
        let mut t = 0;
        for i in 0..26 {
            if i != b {
                t += c[i];
            }
        }
        r += t;
        f |= c[b] > 0;
        c[b] += 1;
    }
    println!("{}", r + if f {1} else {0});
}
