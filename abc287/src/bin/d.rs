use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {
        s: C,
        t: C
    }

    let mut l = 0;
    for i in 0..t.len() {
        if s[i] != '?' && t[i] != '?' && s[i] != t[i] {
            break;
        }
        l += 1;
    }

    let mut r = 0;
    for i in 1..=t.len() {
        if s[s.len() - i] != '?' && t[t.len() - i] != '?' && s[s.len() - i] != t[t.len() - i] {
            break;
        }
        r += 1;
    }

    for i in 0..=t.len() {
        println!("{}", if r >= t.len() - i && l >= i {"Yes"} else {"No"});
    }
}
