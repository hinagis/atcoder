use proconio::{input as I, marker::{Usize1 as U, Chars as C}};

fn main() {
    I! {
        h: usize, w: usize,
        mut i: U, mut j: U,
        c: [C; h],
        x: C
    }
    for x in x {
        match x {
            'L' if  j > 0     && c[i][j - 1] == '.' => {j -= 1},
            'R' if  j < w - 1 && c[i][j + 1] == '.' => {j += 1},
            'U' if  i > 0     && c[i - 1][j] == '.' => {i -= 1},
            'D' if  i < h - 1 && c[i + 1][j] == '.' => {i += 1},
            _ => {},
        }
    }
    println!("{} {}", i + 1, j + 1);
}
