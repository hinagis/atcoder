use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {s: C}
    println!("{}", if s[0] == s[1] {
            let mut c = s[0];
            for i in 2.. {
                if c != s[i] {
                    c = s[i];
                    break;
                }
            }
            c
        } else {
            if s[0] == s[2] {s[1]} else {s[0]}
        });
}
