use proconio::{input as I, marker::Chars as C};
use std::collections::VecDeque as Q;

fn main() {
    I! {
        h: usize,
        w: usize,
        s: [C; h]
    }
    let mut f = vec![vec![[false; 2]; w]; h];
    let mut q = Q::new();
    'outer: for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'S' {
                f[i][j][true as usize] = true;
                q.push_back((i, j, 0, true));
                break 'outer;
            }
        }
    }
    while let Some((i, j, c, d)) = q.pop_front() {
        if s[i][j] == 'G' {
            println!("{}", c);
            return;
        }
        let d = if s[i][j] == '?' {!d} else {d};
        let c = c + 1;
        let k = d as usize;
        if i > 0 {
            calc(&mut q, &mut f, &s, i - 1, j, k, c, d);
        }
        if i < h - 1 {
            calc(&mut q, &mut f, &s, i + 1, j, k, c, d);
        }
        if j > 0 {
            calc(&mut q, &mut f, &s, i, j - 1, k, c, d);
        }
        if j < w - 1 {
            calc(&mut q, &mut f, &s, i, j + 1, k, c, d);
        }
    }
    println!("-1");
}

fn calc(q: &mut Q<(usize, usize, usize, bool)>, f: &mut Vec<Vec<[bool; 2]>>, s: &Vec<Vec<char>>, i: usize, j: usize, k: usize, c: usize, d: bool) {
    if !f[i][j][k] && match s[i][j] {
        '#' => false,
        'o' => d,
        'x' => !d,
        _ => true
    } {
        f[i][j][k] = true;
        q.push_back((i, j, c, d));
    }
}
