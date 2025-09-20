use itertools::iproduct;
use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        h: usize,
        w: usize,
        s: [C; h]
    }
    for (i, j) in iproduct!(0..h, 0..w) {
        if s[i][j] == '.' {continue}
        let b = (i > 0 && s[i - 1][j] == '#') as u8
               + (i < h - 1 && s[i + 1][j] == '#') as u8
               + (j > 0 && s[i][j - 1] == '#') as u8
               + (j < w - 1 && s[i][j + 1] == '#') as u8;
        if b != 2 && b != 4 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
