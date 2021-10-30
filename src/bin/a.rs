use proconio::{input, marker::Chars};

fn main() {
    input! {mut s: Chars}
    s.sort();
    println!("{}", if s[0] == s[2] {1} else {if s[0] == s[1] || s[1] == s[2] {3} else {6}});
}
