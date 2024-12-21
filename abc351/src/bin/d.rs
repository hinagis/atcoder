use proconio::{input as I, marker::Chars as C};
use std::collections::HashSet as H;

fn main() {
    I! {
        h: usize,
        w: usize,
        s: [C; h]
    }
    let mut f = vec![vec![true; w]; h];
    let mut c = vec![vec![H::new(); w]; h];
    for i in 0..h {
        for j in 0..w {
            if f[i][j] && s[i][j] == '.' {
                c[i][j].insert((i, j));
                bfs(&s, &mut c[i][j], &mut f, h, w, i, j);
            }
        }
    }
    println!("{}", c.iter().map(|c| c.iter().map(|c| c.len()).max().unwrap()).max().unwrap());
}

fn bfs(s: &Vec<Vec<char>>, c: &mut H<(usize, usize)>, f: &mut Vec<Vec<bool>>, h: usize, w: usize, i: usize, j: usize) {
    if (i > 0 && s[i - 1][j] == '#') ||
       (j > 0 && s[i][j - 1] == '#') ||
       (i < h - 1 && s[i + 1][j] == '#') ||
       (j < w - 1 && s[i][j + 1] == '#') {
        return;
    }

    if i > 0 && s[i - 1][j] == '.' && c.insert((i - 1, j)) {
        f[i - 1][j] = false;
        bfs(s, c, f, h, w, i - 1, j);
    }
    if j > 0 && s[i][j - 1] == '.' && c.insert((i, j - 1)) {
        f[i][j - 1] = false;
        bfs(s, c, f, h, w, i, j - 1);
    }
    if i < h - 1 && s[i + 1][j] == '.' && c.insert((i + 1, j)) {
        f[i + 1][j] = false;
        bfs(s, c, f, h, w, i + 1, j);
    }
    if j < w - 1 && s[i][j + 1] == '.' && c.insert((i, j + 1)) {
        f[i][j + 1] = false;
        bfs(s, c, f, h, w, i, j + 1);
    }
}
