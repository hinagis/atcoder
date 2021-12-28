use proconio::{input, marker::Bytes};

fn main() {
    input! {s: Bytes}
    let (a, b) = (s[0] - b'0', s[2] - b'0');

    println!("{}", a * b);
}
