use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {t: u32}
    for _ in 0..t {
        I! {
            n: usize,
            c: C,
            x: [i64; n],
            y: [i64; n - 1]
        }
        let (mut s, mut r) = if c[0] == 'S' {
            (0, -x[0])
        } else {
            (-x[0], 0)
        };
        for i in 1..n {
            (s, r) = if c[i] == 'S' {
                (s.max(r + y[i - 1]), s.max(r) - x[i])
            } else {
                (s.max(r + y[i - 1]) - x[i], s.max(r))
            };
        }
        println!("{}", s.max(r));
    }
}
