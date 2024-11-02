use proconio::{input as I, marker::Chars as C};

#[derive(PartialEq)]
enum D {U, D, L, R, N}

fn main() {
    I! {
        h: usize,
        w: usize,
        k: usize,
        s: [C; h]
    }
    let mut t = 0u64;
    let mut c = vec![vec![vec![vec![None; 4]; k]; w]; h];
    for i in 0..h {
        for j in 0..w {
            let mut f = vec![vec![true; w]; h];
            t += dfs(&s, &mut f, &mut c, h, w, i, j, k, D::N);
        }
    }
    println!("{}", t);
}

fn dfs(s: &Vec<Vec<char>>, f: &mut Vec<Vec<bool>>, c: &mut Vec<Vec<Vec<Vec<Option<u64>>>>>, h: usize, w: usize, i: usize, j: usize, k: usize, d: D) -> u64 {
    if s[i][j] == '#' {return 0;}

    if k == 0 {return 1;}
    let k = k - 1;

    f[i][j] = false;

    let mut t = 0;
    if d != D::D && i > 0 {
        t += if let Some(q) = c[i][j][k][0] {
            q
        } else if f[i - 1][j] {
            let q = dfs(s, f, c, h, w, i - 1, j, k, D::U);
            c[i][j][k][0] = Some(q);
            q
        } else {
            0
        };
    }
    if d != D::U && i < h - 1 {
        t += if let Some(q) = c[i][j][k][1] {
            q
        } else if f[i + 1][j] {
            let q = dfs(s, f, c, h, w, i + 1, j, k, D::D);
            c[i][j][k][1] = Some(q);
            q
        } else {
            0
        };
    }
    if d != D::R && j > 0 {
        t += if let Some(q) = c[i][j][k][2] {
            q
        } else if f[i][j - 1] {
            let q = dfs(s, f, c, h, w, i, j - 1, k, D::L);
            c[i][j][k][2] = Some(q);
            q
        } else {
            0
        };
    }
    if d != D::L && j < w - 1 {
        t += if let Some(q) = c[i][j][k][3] {
            q
        } else if f[i][j + 1] {
            let q = dfs(s, f, c, h, w, i, j + 1, k, D::R);
            c[i][j][k][3] = Some(q);
            q
        } else {
            0
        };
    }
    f[i][j] = true;
    t
}