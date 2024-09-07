use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {
        mut s: C,
        t: C
    }
    let mut r = vec![];
    'outer: while s != t {
        for i in 0..t.len() {
            if s[i] > t[i] {
                s[i] = t[i];
                r.push(s.clone());
                continue 'outer;
            }
        }
        for i in (0..t.len()).rev() {
            if s[i] != t[i] {
                s[i] = t[i];
                r.push(s.clone());
                continue 'outer;
            }
        }
    }

    println!("{}", r.len());
    for r in r {
        println!("{}", r.iter().collect::<String>());
    }
}
