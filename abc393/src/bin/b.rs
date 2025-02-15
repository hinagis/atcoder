use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {s: C}
    let n = s. len();
    let mut c = 0;
    for i in 0..n {
        if s[i] == 'A' {
            let mut d = 1;
            while i + d * 2 < n {
                if s[i + d] == 'B' && s[i + d * 2] == 'C' {
                    c += 1;
                }
                d += 1;
            }
        }
    }
    println!("{}", c);
}
