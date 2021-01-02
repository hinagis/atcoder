use proconio::{input, marker::Chars};

fn main() {
    input! {
        a: Chars,
        b: Chars,
    }
    let a = a.iter().fold(0, |s, &e| s + e as u32 - '0' as u32);
    let b = b.iter().fold(0, |s, &e| s + e as u32 - '0' as u32);

    println!("{}", a.max(b));
}
