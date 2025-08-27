use itertools::Itertools;
use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        m: usize,
        s: [C; n]
    }
    let a = (0..n).collect::<Vec<_>>();
    let mut c = vec![0; n];
    for j in 0..m {
        let mut x = Vec::with_capacity(n);
        let mut y = Vec::with_capacity(n);
        for i in 0..n {
            (if s[i][j] == '0' {&mut x} else {&mut y}).push(i);
        }
        let v = if x.len() == 0 || y.len() == 0 {
            &a
        } else if x.len() < y.len() {
            &x
        } else {
            &y
        };
        for &i in v {
            c[i] += 1;
        }
    }
    let mut m = 0;
    let mut w = Vec::with_capacity(n);
    for i in 0..n {
        if c[i] > m {
            m = c[i];
            w.clear();
        }
        if c[i] == m {
            w.push(i + 1);
        }
    }
    println!("{}", w.iter().join(" "));
}
