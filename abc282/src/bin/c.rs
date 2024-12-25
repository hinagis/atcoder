use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {
        _: usize,
        s: C
    }
    let mut f = true;
    for &c in &s {
        f ^= c == '"';
        print!("{}", if f && c == ',' {'.'} else {c});
    }
    println!("");
}
