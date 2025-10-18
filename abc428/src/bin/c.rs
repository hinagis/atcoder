use itertools::Itertools;
use proconio::input as I;

fn main() {
    I! {q: usize}
    let mut v = vec!["No"; q];
    let mut a = 0;
    let mut l = 0i32;
    let mut r = 0i32;
    let mut s = Vec::with_capacity(q);
    for i in 0..q {
        I! {t: u8}
        if t == 1 {
            I! {c: char}
            if c == '(' {
                l += 1;
            } else {
                r += 1;
            }
            s.push(c);
            if l >= r && a + 1 == s.len() {
                a += 1;
            }
        } else {
            let c = s.pop().unwrap();
            if c == '(' {
                l -= 1;
            } else {
                r -= 1;
            }
            a = a.min(s.len());
        }
        if a == s.len() && l == r {
            v[i] = "Yes";
        }
    }
    println!("{}", v.iter().join("\n"));
}
