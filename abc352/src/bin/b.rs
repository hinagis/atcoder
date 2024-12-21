use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        s: C,
        t: C
    }
    let mut p = Vec::with_capacity(s.len());
    let mut i = 0;
    for (j, &c) in t.iter().enumerate() {
        if c == s[i] {
            i += 1;
            p.push(j + 1);
        }
    }

    println!("{}", p.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" "));
}
