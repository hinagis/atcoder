use proconio::{input as I, fastout as F, marker::{Chars as C, Usize1 as U}};

#[F]
fn main() {
    I! {
        n: usize,
        m: usize,
        mut s: C,
        t: C
    }
    let mut f = vec![false; n + 1];
    for _ in 0..m {
        I! {
            l: U,
            r: usize
        }
        f[l] ^= true;
        f[r] ^= true;
    }
    let mut c = true;
    for i in 0..n {
        if f[i] {
            c ^= true;
        }
        if !c {
            s[i] = t[i];
        }
    }
    println!("{}", s.iter().collect::<String>());
}
