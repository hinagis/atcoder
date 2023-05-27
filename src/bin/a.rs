use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        s: C,
        t: C
    }
    for i in 0..n {
        if s[i] == t[i] {continue;}
        if s[i] == '1' && t[i] == 'l' {continue;}
        if s[i] == 'l' && t[i] == '1' {continue;}
        if s[i] == '0' && t[i] == 'o' {continue;}
        if s[i] == 'o' && t[i] == '0' {continue;}
        println!("No");
        return;
    }
    println!("Yes");
}
