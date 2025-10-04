use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {t: u32}
    for _ in 0..t {
        I! {
            n: usize,
            s: C
        }
        if   s.iter().all(|&c| c == '0')
          || s.iter().all(|&c| c == '1') {
            println!("0");
            continue;
        }
        println!("{}", calc(&s, n, '0', '1').min(calc(&s, n, '1', '0')));
    }
}

fn calc(s: &Vec<char>, n: usize, x: char, y: char) -> usize {
    let mut l = 0;
    while s[l] == x {
        l += 1;
    }
    let mut c = l;
    let mut r = l;
    while r < n && s[r] == y {
        r += 1;
    }
    for i in r..n {
        c += if s[i] == x {1} else {2};
    }
    let mut m = c;

    while r < n {
        c += (r - l) * 2;
        while r < n && s[r] == x {
            r += 1;
        }
        l = r;
        while r < n && s[r] == y {
            r += 1;
        }
        c -= (r - l) * 2;
        m = m.min(c);
    }
    m
}