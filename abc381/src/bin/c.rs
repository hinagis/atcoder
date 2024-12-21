use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        s: C
    }
    let mut m = 0;
    for i in 0..n {
        if s[i] == '/' {
            let mut j = 1;
            while j <= i && i + j < n {
                if s[i - j] != '1' || s[i + j] != '2' {
                    break;
                }
                j += 1;
            }
            m = m.max((j - 1) * 2 + 1);
        }
    }
    println!("{}", m);
}
