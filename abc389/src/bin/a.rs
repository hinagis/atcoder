use proconio::{input as I, marker::Bytes as B};

fn main() {
    I! {s: B}
    println!("{}", (s[0] - b'0') * (s[2] - b'0'));
}
