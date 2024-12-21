use itertools::Itertools;
use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {
        n: usize,
        t: C,
        s: [C; n]
    }
    let m = t.len();
    let mut r = vec![];
    for (i, s) in s.iter().enumerate() {
        let k = s.len();
        if k == m {
            let mut u = 0;
            for i in 0..m {
                if t[i] != s[i] {
                    u += 1;
                    if u > 1 {
                        break;
                    }
                }
            }
            if u < 2 {
                r.push(i + 1);
            }
        } else {
            let (a, b) = if k + 1 == m {
                (&t, s)
            } else if k == m + 1 {
                (s, &t)
            } else {
                continue;
            };
            let mut u = 0;
            for i in 0..a.len() - 1 {
                if a[i] != b[i - u] {
                    u += 1;
                    if u > 1 {
                        break;
                    }
                }
            }
            if u == 1 && a.last() != b.last() {
                u += 1;
            }
            if u < 2 {
                r.push(i + 1);
            }
        }
    }
    println!("{}\n{}", r.len(), r.iter().join(" "));
}
