use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {
        n: usize,
        s: C
    }
    for i in 1..n {
        for k in 0..n {
            if k + i >= n || s[k] == s[k + i] {
                println!("{}", k);
                break;
            }
        }
    }
}
