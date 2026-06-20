use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        x: char,
    }
    let x = (x as u8 - b'A') as usize;
    for _ in 0..n {
        I! {s: C}
        if s[x] == 'o' {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
