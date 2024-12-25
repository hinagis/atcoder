use num_integer::Roots;
use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {t: usize}
    for _ in 0..t {
        I! {n: i64}
        let res = factor(n);
        println!("{} {}", res.0, res.1);
    }
}

fn factor(n: i64) -> (i64, i64) {
    fn factor_sub(n: i64, m: i64) -> (i64, i64) {
        let mut c = 0;
        let mut x = n;
        while x % m == 0 {
            c += 1;
            x /= m;
        }
        (c, x)
    }

    let (c, n) = factor_sub(n, 2);
    if c > 0 {
        return if c == 1 {
            (n.sqrt(), 2)
        } else {
            (2, n)
        };
    }
    let mut x = 3;
    let mut m = n;
    while x * x <= m {
        let (c, n) = factor_sub(m, x);
        if c > 0 {
            return if c == 1 {
                (n.sqrt(), x)
            } else {
                (x, n)
            };
        }
        m = n;
        x += 2;
    }
    (0, 0)
}
