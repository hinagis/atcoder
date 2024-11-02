use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        h: usize,
        w: usize,
        k: usize,
        s: [C; h]
    }
    let mut t = 0u64;
    let mut f = vec![vec![true; w]; h];
    for i in 0..h {
        for j in 0..w {
            t += dfs(&s, &mut f, h, w, i, j, k);
        }
    }
    println!("{}", t);
}

fn dfs(s: &Vec<Vec<char>>, f: &mut Vec<Vec<bool>>,h: usize, w: usize, i: usize, j: usize, k: usize) -> u64 {
    if s[i][j] == '#' {return 0;}

    if k == 0 {return 1;}
    let k = k - 1;

    f[i][j] = false;
    let t = if i > 0     && f[i - 1][j] {dfs(s, f, h, w, i - 1, j, k)} else {0}
               + if i < h - 1 && f[i + 1][j] {dfs(s, f, h, w, i + 1, j, k)} else {0}
               + if j > 0     && f[i][j - 1] {dfs(s, f, h, w, i, j - 1, k)} else {0}
               + if j < w - 1 && f[i][j + 1] {dfs(s, f, h, w, i, j + 1, k)} else {0};
    f[i][j] = true;
    t
}