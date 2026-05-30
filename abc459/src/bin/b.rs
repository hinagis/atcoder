use proconio::{input as I, marker::Bytes as B};

fn main() {
    I! {n: usize}
    let mut c = Vec::with_capacity(n);
    for _ in 0..n {
        I! {s: B}
        let b = s[0] - b'a' - match s[0] {b'z'.. => 2, b's'.. => 1, _ => 0};
        c.push((b / 3 + b'2') as char)
    }
    println!("{}", c.iter().collect::<String>());
}
