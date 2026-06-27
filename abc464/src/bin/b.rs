use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {
        h: usize, w: usize,
        c: [C; h]
    }
    let mut y = vec![true; h];
    for i in 0..h {
        if c[i].iter().any(|&c| c == '#') {break}
        y[i] = false;
    }
    for i in (0..h).rev() {
        if c[i].iter().any(|&c| c == '#') {break}
        y[i] = false;
    }
    let mut x = vec![true; w];
    for j in 0..w {
        if c.iter().any(|c| c[j] == '#') {break}
        x[j] = false;
    }
    for j in (0..w).rev() {
        if c.iter().any(|c| c[j] == '#') {break}
        x[j] = false;
    }
    for i in 0..h {
        if !y[i] {continue}
        for j in 0..w {
            if !x[j] {continue}
            print!("{}", c[i][j]);
        }
        println!("");
    }
}
