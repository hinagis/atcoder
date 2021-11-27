use proconio::{input, marker::Bytes};

fn main() {
    input! {
        a: Bytes,
        b: Bytes,
    }
    let a = a.iter().rev().map(|&e| e - b'0').collect::<Vec<_>>();
    let b = b.iter().rev().map(|&e| e - b'0').collect::<Vec<_>>();
    let n = a.len().min(b.len());

    for i in 0..n {
        if a[i] + b[i] >= 10 {
            println!("Hard");
            return;
        }
    }
    println!("Easy");
}
