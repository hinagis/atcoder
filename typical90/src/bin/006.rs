use proconio::{input as I, marker::Bytes as B};

fn main() {
    I! {
        n: usize,
        k: usize,
        s: B,
    }

    let mut next = vec![vec![None; 26]; n];
    next[n - 1][(s[n - 1] - b'a') as usize] = Some(n - 1);
    for (i, &c) in (s.iter().enumerate()).rev().skip(1) {
        for b in 0..26 {
            next[i][b] = next[i + 1][b];
        }
        next[i][(c - b'a') as usize] = Some(i);
    }

    let mut t = String::with_capacity(k);
    let mut p = 0;
    for i in 0..k {
        for b in 0..26 {
            if let Some(q) = next[p][b] {
                if n - q + i >= k {
                    t.push((b as u8 + b'a') as char);
                    p = q + 1;
                    break;
                }
            }
        }
    }
    println!("{}", t);
}
