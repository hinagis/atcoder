use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    if k >= n - 1 {
        println!("{}", a.iter()
            .map(|_| n.to_string())
            .collect::<Vec<_>>()
            .join(" ")
        );
        return;
    }

    let mut b = a;
    for _ in 0..k {
        let mut s = vec![0isize; n + 1];
        for i in 0..n {
            s[max(b[i], i) - b[i]] += 1;
            s[min(n, i + b[i] + 1)] -= 1;
        }
        b = vec![0; n];
        let mut t = 0;
        let mut f = true;
        for i in 0..n {
            t += s[i];
            b[i] = t as usize;
            if b[i] < n {
                f = false;
            }
        }
        if f {
            break;
        }
    }

    let b = b.iter()
        .map(|bi| bi.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", b);
}
