use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        a: u64,
        b: u64,
        s: C
    }

    let mut r = std::u64::MAX;
    for i in 0..n {
        let mut c = a * i as u64;
        for j in 0..n / 2 {
            if s[(i + j) % n] != s[(i + n - 1 - j) % n] {
                c += b;
            }
        }
        r = r.min(c);
    }
    println!("{}", r);
}
