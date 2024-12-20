use itertools::iproduct;
use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        h: usize,
        w: usize,
        d: u32,
        s: [C; h]
    }
    let mut q = std::collections::VecDeque::new();
    for (i, j) in iproduct!(0..h, 0..w) {
        if s[i][j] == 'H' {
            q.push_back((i, j, d + 1));
        }
    }
    let mut f = vec![vec![0; w]; h];
    while let Some((i, j, r)) = q.pop_front() {
        f[i][j] = f[i][j].max(r);
        if i > 0     && s[i - 1][j] == '.' && r > f[i - 1][j] {q.push_back((i - 1, j, r - 1));}
        if j > 0     && s[i][j - 1] == '.' && r > f[i][j - 1] {q.push_back((i, j - 1, r - 1));}
        if i < h - 1 && s[i + 1][j] == '.' && r > f[i + 1][j] {q.push_back((i + 1, j, r - 1));}
        if j < w - 1 && s[i][j + 1] == '.' && r > f[i][j + 1] {q.push_back((i, j + 1, r - 1));}
    }
    println!("{}", f.iter().map(|e| e.iter().filter(|&&c| c > 0).count()).sum::<usize>());
}
