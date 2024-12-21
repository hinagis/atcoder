use proconio::{input as I, fastout as F, marker::{Usize1 as U, Chars as C}};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize,
        s: C
    }
    let mut c = vec![0; n];
    let mut p = s[0];
    for i in 1..n {
        c[i] = c[i - 1] + if s[i] == p {1} else {0};
        p = s[i];
    }
    for _ in 0..q {
        I! {
            l: U,
            r: U
        }
        println!("{}", c[r] - c[l]);
    }
}
