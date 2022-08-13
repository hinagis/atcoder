use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {mut s: C}

    let mut c = 0;
    c += swap(&mut s, 'a', 0);
    c += swap(&mut s, 't', 1);
    c += swap(&mut s, 'c', 2);
    c += swap(&mut s, 'o', 3);
    c += swap(&mut s, 'd', 4);
    c += swap(&mut s, 'e', 5);

    println!("{}", c);
}

fn swap(s: &mut Vec<char>, t: char, p: usize) -> u32 {
    let mut c = 0;
    let mut i = p;
    while s[i] != t {
        i += 1
    }
    while i > p {
        c += 1;
        s.swap(i - 1, i);
        i -= 1;
    }
    c
}
