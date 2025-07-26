use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {t: u32}
    for _ in 0..t {
        I! {
            n: usize,
            mut s: C
        }
        s.insert(0, '0');
        let m = 2usize.pow(n as u32);
        let mut f = vec![false; m];
        f[0] = true;
        for i in 0..m {
            if !f[i] {continue}
            for j in 0..n {
                let k = i | (1 << j);
                if s[k] == '0' {
                    f[k] = true;
                }
            }
        }

        println!("{}", if f[m - 1] { "Yes" } else { "No" });
    }
}
