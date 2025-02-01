use itertools::iproduct;
use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        h: usize,
        w: usize,
        s: [C; h]
    }
    let (mut t, mut b, mut l, mut r) = (h, 0, w, 0);
    for (i, j) in iproduct!(0..h, 0..w) {
        if s[i][j] == '#' {
            t = t.min(i);
            b = b.max(i);
            l = l.min(j);
            r = r.max(j);
        }
    }
    for (i, j) in iproduct!(t..=b, l..=r) {
        if s[i][j] == '.' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
