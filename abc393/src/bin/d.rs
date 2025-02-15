use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        s: C
    }
    let mut c = std::collections::VecDeque::new();
    let mut p = 0;
    for i in 0..n {
        if s[i] == '1' {
            p = i;
            break;
        }
    }
    for i in p + 1..n {
        if s[i] == '1' {
            c.push_back(i - p - 1);
            p = i;
        }
    }
    let mut r = 0;
    let mut k = 1;
    while let Some(h) = c.pop_front() {
        r += h * k;
        if let Some(t) = c.pop_back() {
            r += t * k;
        }
        k += 1;
    }
    println!("{}", r);
}
