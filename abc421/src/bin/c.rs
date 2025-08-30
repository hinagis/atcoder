use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        s: C
    }
    let mut a = 0;
    let mut j = 0;
    for i in 0..2 * n {
        if s[i] == 'A' {
            a += ((i as i64) - j).abs();
            j += 2;
        }
    }
    let mut b = 0;
    let mut j = 0;
    for i in 0..2 * n {
        if s[i] == 'B' {
            b += ((i as i64) - j).abs();
            j += 2;
        }
    }
    println!("{}", a.min(b));
}
