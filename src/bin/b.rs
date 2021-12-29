use proconio::{input, fastout, marker::{Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        l: Usize1,
        r: Usize1,
        s: Chars,
    }
    for i in 0..l {
        print!("{}", s[i]);
    }
    for i in (l..=r).rev() {
        print!("{}", s[i]);
    }
    for i in r + 1..s.len() {
        print!("{}", s[i]);
    }
    println!("");
}
