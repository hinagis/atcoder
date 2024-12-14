use itertools::{iproduct, Itertools};
use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        h: usize,
        w: usize,
        d: i32,
        s: [C; h]
    }

    let mut f = vec![];
    for (i, j) in iproduct!(0..h, 0..w) {
        if s[i][j] == '.' {
            f.push((i as i32, j as i32));
        }
    }

    let mut s = 0;
    for (&u, &v) in f.iter().tuple_combinations() {
        let mut t = 0;
        for &(i, j) in f.iter() {
            if (u.0 - i).abs() + (u.1 - j).abs() <= d ||
               (v.0 - i).abs() + (v.1 - j).abs() <= d {
                t += 1;
            }
        }
        s = s.max(t);
    }
    println!("{}", s);
}
