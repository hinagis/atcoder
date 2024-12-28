use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        k: usize,
        s: C,
        t: C
    }
    let mut r = "Yes";
    let mut i = 0;
    let mut c = 0;
    println!("{}", if s.len() == t.len() {
        for i in 0..s.len() {
            if s[i] != t[i] {
                if c < k {
                    c += 1;
                } else {
                    r = "No";
                    break;
                }
            }
        }
        r
    } else if s.len() + 1 == t.len() {
        while i < s.len() && i + c < t.len() {
            if s[i] != t[i + c] {
                c += 1;
                if c > k {
                    r = "No";
                    break;
                }
            } else {
                i += 1;
            }
        }
        r
    } else if s.len() == t.len() + 1 {
        while i + c < s.len() && i < t.len() {
            if s[i + c] != t[i] {
                c += 1;
                if c > k {
                    r = "No";
                    break;
                }
            } else {
                i += 1;
            }
        }
        r
    } else {
        "No"
    });
}
