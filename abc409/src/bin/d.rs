use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {t: usize}
    for _ in 0..t {
        I! {
            n: usize,
            s: C
        }
        let mut t = Vec::with_capacity(n);
        for l in 0..n - 1 {
            if s[l] > s[l + 1] {
                for r in l + 1..n {
                    if s[l] < s[r] {
                        break;
                    }
                    t.push(s[r]);
                }
            }
            t.push(s[l]);
            if t.len() > l + 1 {
                for j in t.len()..n {
                    t.push(s[j]);
                }
                break;
            }
        }
        if t.len() < n {
            t.push(s[n - 1]);
        }
        println!("{}", t.iter().collect::<String>());
    }
}
