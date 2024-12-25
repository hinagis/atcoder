use proconio::{input as I, marker::Chars};

fn main() {
    I! {mut s: Chars}
    s.sort();
    println!("{}", s.iter().collect::<String>());
}
