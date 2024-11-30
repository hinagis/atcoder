use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        _: usize,
        d: usize,
        mut s: C
    }
    let mut i = s.len() - 1;
    for _ in 0..d {
        while s[i] == '.' {
            i -= 1;
        }
        s[i] = '.';
    }
    println!("{}", s.iter().collect::<String>());
}
