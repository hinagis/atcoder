use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {s: C}
    for i in 0..s.len() / 2 {
        print!("{}{}", s[2 * i + 1], s[2 * i]);
    }
    println!();
}
