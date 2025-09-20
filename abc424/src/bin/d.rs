use itertools::iproduct;
use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {t: u32}
    for _ in 0..t {
        I! {
            h: usize,
            w: usize,
            mut s: [C; h]
        }
        let mut c = 0;
        for (i, j) in iproduct!(1..h - 1, 1..w - 1) {
            if s[i][j] != '#' {continue}
            if (s[i - 1][j] == '#'
                && ((s[i - 1][j - 1] == '#' && s[i][j - 1] == '#')
                 || (s[i - 1][j + 1] == '#' && s[i][j + 1] == '#')
                ))
               ||(
               (s[i + 1][j] == '#')
                && ((s[i + 1][j - 1] == '#' && s[i][j - 1] == '#')
                 || (s[i + 1][j + 1] == '#' && s[i][j + 1] == '#')
                ))
            {
                s[i][j] = '.';
                c += 1;
            }
        }
        println!("{}", c);
    }
}
