use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let n = s.len() - t.len();
    let mut r = t.len();
    for i in 0..=n {
        let mut v = 0;
        for j in 0..t.len() {
            if s[i + j] != t[j] {
                v += 1;
            }
        }
        r = r.min(v);
    }
    println!("{}", r);
}
