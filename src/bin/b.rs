use proconio::{input, marker::Bytes};

fn main() {
    input! {x: Bytes}
    let x: Vec<u8> = x.iter().map(|c| c - b'0').collect();

    println!("{}", if (x[0] == x[1] && x[1] == x[2] && x[2] == x[3])
    || ((x[0] + 1) % 10 == x[1] && (x[1] + 1) % 10 == x[2] && (x[2] + 1) % 10 == x[3]) {
        "Weak"
    } else {
        "Strong"
    });
}
