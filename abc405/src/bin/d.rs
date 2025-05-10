use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        h: usize,
        w: usize,
        s: [C; h]
    }
    let mut t = s.clone();
    let mut c = vec![vec![0; w]; h];
    let mut q = std::collections::VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            match s[i][j] {
                '.' => c[i][j] = u32::MAX,
                'E' => q.push_back((i, j, 0)),
                _ => ()
            }
        }
    }
    while let Some((i, j, p)) = q.pop_front() {
        let n = p + 1;
        if j > 0 && n < c[i][j - 1] {
            c[i][j - 1] = n;
            t[i][j - 1] = '>';
            q.push_back((i, j - 1, n));
        }
        if i > 0 && n < c[i - 1][j] {
            c[i - 1][j] = n;
            t[i - 1][j] = 'v';
            q.push_back((i - 1, j, n));
        }
        if j < w - 1 && n < c[i][j + 1] {
            c[i][j + 1] = n;
            t[i][j + 1] = '<';
            q.push_back((i, j + 1, n));
        }
        if i < h - 1 && n < c[i + 1][j] {
            c[i + 1][j] = n;
            t[i + 1][j] = '^';
            q.push_back((i + 1, j, n));
        }
    }
    println!("{}", t.iter().map(|x| x.iter().collect::<String>()).collect::<Vec<_>>().join("\n"));
}
