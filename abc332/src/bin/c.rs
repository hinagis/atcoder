use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        m: usize,
        s: C
    }
    let mut r = 0;
    let mut c = 0;
    let mut x = 0;
    for i in (0..n).rev() {
        match s[i] {
            '0' => {
                r = 0;
                x = x.max(c);
                c = 0;
            },
            '1' => {
                if r >= m {
                    c += 1;
                } else {
                    r += 1;
                }
            },
            _ => {
                c += 1;
            }
        }
    }
    x = x.max(c);
    println!("{}", x);
}
