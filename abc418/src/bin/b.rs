use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {s: C}
    let n = s.len();
    let mut c = vec![0; n + 1];
    for i in 0..n {
        c[i + 1] = c[i];
        if s[i] == 't' {
            c[i + 1] += 1;
        }
    }
    let mut m = 0f64;
    for i in 0..n {
        for j in i + 3..=n {
            let d = c[j] - c[i];
            if d > 2 {
                m = m.max((d - 2) as f64 / (j - i - 2) as f64);
            }
        }
    }
    println!("{}", m);
}
