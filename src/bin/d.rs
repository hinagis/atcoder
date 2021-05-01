use proconio::{input, fastout, marker::Chars};

#[fastout]
fn main() {
    input! {s: Chars}

    let mut t = Vec::with_capacity(5 * 100_000);
    let mut f = false;
    for i in 0..s.len() {
        if s[i] == 'R' {
            f ^= true;
        } else {
            if f {
                if t.len() == 0 || t[0] != s[i] {
                    t.insert(0, s[i]);
                } else {
                    t.remove(0);
                }
            } else {
                if t.len() == 0 || t[t.len() - 1] != s[i] {
                    t.push(s[i]);
                } else {
                    t.remove(t.len() - 1);
                }
            }
        }
    }
    if f {
        for i in (0..t.len()).rev() {
            print!("{}", t[i])
        }
    } else {
        for i in 0..t.len() {
            print!("{}", t[i])
        }
    }
    println!("");
}
