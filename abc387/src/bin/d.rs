use itertools::iproduct;
use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        h: usize,
        w: usize,
        s: [C; h]
    }
    let mut r = vec![vec![[u32::MAX; 2]; w]; h];
    let f = |c| {
        for (i, j) in iproduct!(0..h, 0..w) {
            if s[i][j] == c {
                return (i, j);
            }
        }
        (0, 0)
    };
    let ((i, j), g) = (f('S'), f('G'));
    r[i][j] = [0, 0];
    let mut q = std::collections::VecDeque::new();
    if i > 0 && s[i - 1][j] != '#' {
        r[i - 1][j][0] = 1;
        q.push_back((i - 1, j, 0, 1));
    }
    if i < h - 1 && s[i + 1][j] != '#' {
        r[i + 1][j][0] = 1;
        q.push_back((i + 1, j, 0, 1));
    }
    if j > 0 && s[i][j - 1] != '#' {
        r[i][j - 1][1] = 1;
        q.push_back((i, j - 1, 1, 1));
    }
    if j < w - 1 && s[i][j + 1] != '#' {
        r[i][j + 1][1] = 1;
        q.push_back((i, j + 1, 1, 1));
    }
    let mut m = u32::MAX;
    while let Some((i, j, d, c)) = q.pop_front() {
        r[i][j][d] = r[i][j][d].min(c);
        if (i, j) == g {
            m = m.min(c);
        } else {
            let c = c + 1;
            let d = d ^ 1;
            if d == 0 {
                if i > 0     && s[i - 1][j] != '#' && c < r[i - 1][j][d] {
                    r[i - 1][j][d] = r[i - 1][j][d].min(c);
                    q.push_back((i - 1, j, d, c));
                }
                if i < h - 1 && s[i + 1][j] != '#' && c < r[i + 1][j][d] {
                    r[i + 1][j][d] = r[i + 1][j][d].min(c);
                    q.push_back((i + 1, j, d, c));
                }
            } else {
                if j > 0     && s[i][j - 1] != '#' && c < r[i][j - 1][d] {
                    r[i][j - 1][d] = r[i][j - 1][d].min(c);
                    q.push_back((i, j - 1, d, c));
                }
                if j < w - 1 && s[i][j + 1] != '#' && c < r[i][j + 1][d] {
                    r[i][j + 1][d] = r[i][j + 1][d].min(c);
                    q.push_back((i, j + 1, d, c));
                }
            }
        }
    }
    if m == u32::MAX {
        println!("-1");
    } else {
        println!("{}", m);
    }
}
