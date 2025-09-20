use proconio::{input as I, marker::Bytes as B};

fn main() {
    I! {s: B}
    let (w, s) = (s[0] - b'0', s[2] - b'0');
    println!("{}-{}", w + s / 8, 1 + s % 8);
}
