use proconio::{input as I, marker::{Chars, Usize1 as U1}};
fn main() {
    I! {
        n: usize,
        a: (U1, U1),
        b: (U1, U1),
        s: [Chars; n]
    }
    let mut q = std::collections::VecDeque::new();
    let mut f = vec![vec![[u32::max_value() - 1; 4]; n]; n];
    if a.0 + 1 < n && a.1 + 1 < n && s[a.0 + 1][a.1 + 1] == '.' {
        f[a.0 + 1][a.1 + 1][0] = 1;
        q.push_back(((a.0 + 1, a.1 + 1), 0));
    }
    if a.0 + 1 < n && a.1 >= 1 && s[a.0 + 1][a.1 - 1] == '.' {
        f[a.0 + 1][a.1 - 1][1] = 1;
        q.push_back(((a.0 + 1, a.1 - 1), 1));
    }
    if a.0 >= 1 && a.1 >= 1 && s[a.0 - 1][a.1 - 1] == '.' {
        f[a.0 - 1][a.1 - 1][2] = 1;
        q.push_back(((a.0 - 1, a.1 - 1), 2));
    }
    if a.0 >= 1 && a.1 + 1 < n && s[a.0 - 1][a.1 + 1] == '.' {
        f[a.0 - 1][a.1 + 1][3] = 1;
        q.push_back(((a.0 - 1, a.1 + 1), 3));
    }

    while let Some(((x, y), d)) = q.pop_front() {
        if (x, y) == b {
            println!("{}", f[x][y][d]);
            return;
        }
        if x + 1 < n && y + 1 < n && s[x + 1][y + 1] == '.' {
            let c = f[x][y][d] + if d == 0 {0} else {1};
            if c < f[x + 1][y + 1][0] {
                f[x + 1][y + 1][0] = c;
                if d == 0 {
                    q.push_front(((x + 1, y + 1), 0));
                } else {
                    q.push_back(((x + 1, y + 1), 0));
                }
            }
        }
        if x + 1 < n && y >= 1 && s[x + 1][y - 1] == '.' {
            let c = f[x][y][d] + if d == 1 {0} else {1};
            if c < f[x + 1][y - 1][1] {
                f[x + 1][y - 1][1] = c;
                if d == 1 {
                    q.push_front(((x + 1, y - 1), 1));
                } else {
                    q.push_back(((x + 1, y - 1), 1));
                }
            }
        }
        if x >= 1 && y >= 1 && s[x - 1][y - 1] == '.' {
            let c = f[x][y][d] + if d == 2 {0} else {1};
            if c < f[x - 1][y - 1][2] {
                f[x - 1][y - 1][2] = c;
                if d == 2 {
                    q.push_front(((x - 1, y - 1), 2));
                } else {
                    q.push_back(((x - 1, y - 1), 2));
                }
            }
        }
        if x >= 1 && y + 1 < n && s[x - 1][y + 1] == '.' {
            let c = f[x][y][d] + if d == 3 {0} else {1};
            if c < f[x - 1][y + 1][3] {
                f[x - 1][y + 1][3] = c;
                if d == 3 {
                    q.push_front(((x - 1, y + 1), 3));
                } else {
                    q.push_back(((x - 1, y + 1), 3));
                }
            }
        }
    }
    println!("-1");
}
