use proconio::{input, fastout, marker::Chars};

#[fastout]
fn main() {
    input! {s: Chars}

    let mut t = std::collections::VecDeque::new();
    let mut f = false;
    for i in 0..s.len() {
        if s[i] == 'R' {
            f ^= true;
        } else {
            if f {
                if t.len() == 0 || *t.front().unwrap() != s[i] {
                    t.push_front(s[i]);
                } else {
                    t.pop_front();
                }
            } else {
                if t.len() == 0 || *t.back().unwrap() != s[i] {
                    t.push_back(s[i]);
                } else {
                    t.pop_back();
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
