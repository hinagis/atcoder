use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {s: C}
    println!("{}", if (|| {
        if s.len() % 2 != 0 {
            return false;
        }
        let mut h = std::collections::HashSet::new();
        for i in 0..s.len() / 2 {
            if ! h.insert(s[i * 2]) {
                return false;
            }
            if s[i * 2] != s[i * 2 + 1] {
                return false;
            }
        }
        true
    })() {"Yes"} else {"No"});
}
