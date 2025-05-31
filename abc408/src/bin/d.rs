use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {t: usize}
    for _ in 0..t {
        I! {
            n: usize,
            s: C
        }
        let mut c = vec![];
        let mut p = '0';
        let mut l = 0;
        for i in 0..n {
            if s[i] != p {
                c.push(i - l);
                l = i;
            }
        }
        c.push(n - l);

        println!("{}", n);
    }
}
