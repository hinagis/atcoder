use proconio::{input, fastout, marker::Chars};

#[fastout]
fn main() {
    input! {s: Chars}

    for i in 0..s.len() - 2 {
        print!("{}", s[i]);
    }
    println!("{}", match s[s.len() - 1] {
        '0'..='2' => "-",
        '7'..='9' => "+",
        _ => ""
    });
}
