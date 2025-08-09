use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        t: C
    }
    let mut c = 0;
    let mut p = 0;
    let mut i = 0;
    while i < n {
        if t[i] == '0' {
            let mut j = i + 1;
            if j < n {
                if t[j] == '0' {
                    p += 1;
                    i = j + 1;
                } else {
                    while j < n && t[j] == '1' {
                        j += 1;
                    }
                    let d = j - i - 1;
                    c += d * (d + 1) / 2;
                    if j < n {
                        p += 1;
                    }
                    i = j + 1;
                }
            }
        } else {
            p += 1;
            i += 1;
        }
    }
    println!("{}", c + p * (p + 1) / 2);
}
