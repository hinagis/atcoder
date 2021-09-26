use proconio::{input, marker::Bytes};

fn main() {
    input! {
        k: u64,
        a: Bytes,
        b: Bytes,
    }
    let a = a.iter().rev().enumerate().fold(0, |s, (i, e)| s + ((e - b'0') as u64) * k.pow(i as u32));
    let b = b.iter().rev().enumerate().fold(0, |s, (i, e)| s + ((e - b'0') as u64) * k.pow(i as u32));

    println!("{}", a * b);
}
