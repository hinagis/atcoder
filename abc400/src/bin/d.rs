use proconio::{input as I, marker::{Chars as C, Usize1 as U}};

fn main() {
    I! {
        h: usize,
        w: usize,
        s: [C; h],
        a: U,
        b: U,
        c: U,
        d: U
    }
    let mut f = vec![vec![u32::MAX; w]; h];
    f[a][b] = 0;
    let mut q = std::collections::VecDeque::new();
    q.push_back((a, b, 0));
    while let Some((i, j, t)) = q.pop_front() {
        if i == c && j == d {
            continue;
        }
        if i > 0 {
            let i = i - 1;
            if f[i][j] > t {
                if s[i][j] == '.' {
                    f[i][j] = t;
                    q.push_back((i, j, t));
                } else {
                    f[i][j] = t + 1;
                    q.push_back((i, j, t + 1));

                    if i > 0 {
                        let i = i - 1;
                        if f[i][j] > t {
                            f[i][j] = t + 1;
                            q.push_back((i, j, t + 1));
                        }
                    }
                }
            }
        }
        if j > 0 {
            let j = j - 1;
            if f[i][j] > t {
                if s[i][j] == '.' {
                    f[i][j] = t;
                    q.push_back((i, j, t));
                } else {
                    f[i][j] = t + 1;
                    q.push_back((i, j, t + 1));

                    if j > 0 {
                        let j = j - 1;
                        if f[i][j] > t {
                            f[i][j] = t + 1;
                            q.push_back((i, j, t + 1));
                        }
                    }
                }
            }
        }
        if i < h - 1 {
            let i = i + 1;
            if f[i][j] > t {
                if s[i][j] == '.' {
                    f[i][j] = t;
                    q.push_back((i, j, t));
                } else {
                    f[i][j] = t + 1;
                    q.push_back((i, j, t + 1));

                    if i < h - 1  {
                        let i = i + 1;
                        if f[i][j] > t {
                            f[i][j] = t + 1;
                            q.push_back((i, j, t + 1));
                        }
                    }
                }
            }
        }
        if j < w - 1 {
            let j = j + 1;
            if f[i][j] > t {
                if s[i][j] == '.' {
                    f[i][j] = t;
                    q.push_back((i, j, t));
                } else {
                    f[i][j] = t + 1;
                    q.push_back((i, j, t + 1));

                    if j < w - 1  {
                        let j = j + 1;
                        if f[i][j] > t {
                            f[i][j] = t + 1;
                            q.push_back((i, j, t + 1));
                        }
                    }
                }
            }
        }
    }

    println!("{}", f[c][d]);
}
