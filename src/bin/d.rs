use proconio::{input as I, marker::Chars};
fn main() {
    I! {
        n: usize,
        mut x: usize,
        s: Chars
    }
    let mut r = 0;
    for i in 0..n {
        if r == 0 {
            if s[i] == 'L' {
                if x * 2 > 1_000_000_000_000_000_000 {
                    r += 1
                } else {
                    x *= 2
                }
            } else if s[i] == 'R' {
                if x * 2 + 1 > 1_000_000_000_000_000_000 {
                    r += 1
                } else {
                    x = x * 2 + 1
                }
            } else {
                x /= 2
            }
        } else {
            if s[i] == 'U' {
                r -= 1
            } else {
                r += 1
            }
        }
    }
    println!("{}", x);
}
