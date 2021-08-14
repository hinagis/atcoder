use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h]
    }
    let f = vec![vec![true; w]; h];
    let mut m = -1;
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '.' {
                let t = if i > 0     {dfs(&c, h, w, f.clone(), (i, j), (i - 1, j), 1)} else {-1};
                let l = if j > 0     {dfs(&c, h, w, f.clone(), (i, j), (i, j - 1), 1)} else {-1};
                let b = if i < h - 1 {dfs(&c, h, w, f.clone(), (i, j), (i + 1, j), 1)} else {-1};
                let r = if j < w - 1 {dfs(&c, h, w, f.clone(), (i, j), (i, j + 1), 1)} else {-1};
                m = m.max(t).max(l).max(b).max(r);
            }
        }
    }

    println!("{}", m);
}

fn dfs(c: &Vec<Vec<char>>, h: usize, w: usize, mut f: Vec<Vec<bool>>, s: (usize, usize), p: (usize, usize), d: i32) -> i32 {
    let (i, j) = p;
    if p == s {
        if d >= 3 {d} else {-1}
    } else if c[i][j] != '.' {
        -1
    } else {
        f[i][j] = false;
        let t = if i > 0     && f[i - 1][j] {dfs(&c, h, w, f.clone(), s, (i - 1, j), d + 1)} else {-1};
        let l = if j > 0     && f[i][j - 1] {dfs(&c, h, w, f.clone(), s, (i, j - 1), d + 1)} else {-1};
        let b = if i < h - 1 && f[i + 1][j] {dfs(&c, h, w, f.clone(), s, (i + 1, j), d + 1)} else {-1};
        let r = if j < w - 1 && f[i][j + 1] {dfs(&c, h, w, f.clone(), s, (i, j + 1), d + 1)} else {-1};
        t.max(l).max(b).max(r)
    }
}
