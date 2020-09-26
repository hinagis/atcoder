use proconio::{input, marker::Chars};
use std::cmp::Ordering::{Greater, Equal, Less};

fn main() {
    input! {
        _n: usize,
        mut s: Chars,
    }
    s.sort_by(|a, b| match (a, b) {
        ('J', _) => Less,
        (_, 'I') => Less,
        ('I', _) => Greater,
        (_, 'J') => Greater,
        _ => Equal,
    });
    let s: String = s.iter().collect();

    println!("{}", s);
}
