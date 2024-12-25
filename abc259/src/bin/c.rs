use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        s: C,
        t: C,
    }
    let mut i = 0;
    let mut j = 0;
    while i < s.len() && j < t.len() {
        if s[i] != t[j] {
            println!("No");
            return;
        }

        let c = s[i];
        let mut a = 0;
        while i < s.len() && s[i] == c {
            a += 1;
            i += 1;
        }

        let c = t[j];
        let mut b = 0;
        while j < t.len() && t[j] == c {
            b += 1;
            j += 1;
        }

        if a > b || (a == 1 && b > 1) {
            println!("No");
            return;
        }
    }

    println!("{}", if i == s.len() && j == t.len() {"Yes"} else {"No"});
}
