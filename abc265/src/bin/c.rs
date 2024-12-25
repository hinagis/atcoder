use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        h: usize,
        w: usize,
        g: [C; h]
    }
    let (mut i, mut j) = (0, 0);

    let mut f = vec![vec![false; w]; h];
    loop {
        if f[i][j] {
            println!("-1");
            return;
        }
        f[i][j] = true;
        match g[i][j] {
            'U' => {if i > 0 {i -= 1} else {break}},
            'D' => {if i < h - 1 {i += 1} else {break}},
            'L' => {if j > 0 {j -= 1} else {break}},
            'R' => {if j < w - 1 {j += 1} else {break}},
            _ => ()
        }
    }
    println!("{} {}", i + 1, j + 1);
}
