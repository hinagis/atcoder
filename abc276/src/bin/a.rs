use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {s: C}
    for (i, &c) in s.iter().enumerate().rev() {
        if c == 'a' {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
