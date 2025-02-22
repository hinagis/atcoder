use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {mut s: C}
    let n = s.len();
    for i in (0..n).rev().skip(1) {
        if s[i] == 'W' && s[i + 1] == 'A' {
            s[i] = 'A';
            s[i + 1] = 'C';
        }
    }
    println!("{}", s.iter().collect::<String>());
}
