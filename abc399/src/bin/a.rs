use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        s: C,
        t: C
    }
    println!("{}", (0..n).filter(|&i| s[i] != t[i]).count());
}
