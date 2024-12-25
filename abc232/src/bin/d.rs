use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h]
    }
    let mut f = vec![vec![true; w]; h];
    println!("{}", dfs(&c, h - 1, w - 1, &mut f, 0, 0, 1));
}

fn dfs(c: &Vec<Vec<char>>, h: usize, w: usize, f: &mut Vec<Vec<bool>>, i: usize, j: usize, d: usize) -> usize {
    let x = if j < w && f[i][j + 1] && c[i][j + 1] != '#' {f[i][j + 1] = false; dfs(c, h, w, f, i, j + 1, d + 1)} else {d};
    let y = if i < h && f[i + 1][j] && c[i + 1][j] != '#' {f[i + 1][j] = false; dfs(c, h, w, f, i + 1, j, d + 1)} else {d};
    x.max(y)
}
