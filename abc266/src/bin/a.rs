use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {s: C}
    println!("{}", s[s.len() / 2]);
}
