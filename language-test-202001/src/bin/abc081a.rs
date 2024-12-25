use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    println!("{}", s.iter().filter(|&c| *c == '1').count());
}
