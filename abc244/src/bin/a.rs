use proconio::{input as I, marker::Chars};

fn main() {
    I! {
        n: usize,
        s: Chars
    }
    println!("{}", s[n - 1]);
}
