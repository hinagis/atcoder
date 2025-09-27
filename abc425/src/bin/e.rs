use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        q: u32,
        m: i64
    }
    for _ in 0..q {
        I! {
            n: usize,
            c: [i64; n]
        }
        let mut r = c.iter().sum::<i64>();
        r %= m;
        r = fact(m, r);
        for i in 0..n {
            r = div(m, r, fact(m, c[i]));
        }
        println!("{}", r);
    }
}

pub fn inverse(m: i64, x: i64) -> i64 {
    fn extended_gcd(a: i64, b: i64) -> (i64, i64) {
        if (a, b) == (1, 0) {
            (1, 0)
        } else {
            let (x, y) = extended_gcd(b, a % b);
            (y, x - (a / b) * y)
        }
    }

    let (x, _y) = extended_gcd(x, m);
    (m + x) % m
}
fn div(m: i64, x: i64, y: i64) -> i64 {
    x * inverse(m, y) % m
}

fn fact(m: i64, n: i64) -> i64 {
    (1..=n).fold(1, |x, y| (x * y) % m)
}
