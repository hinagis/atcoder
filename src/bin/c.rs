use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        s: C,
        t: C,
    }

    for (i, &c) in s.iter().enumerate() {
        if c != t[i] {
            println!("{}", i + 1);
            return;
        }
    }
    println!("{}", t.len());
}
