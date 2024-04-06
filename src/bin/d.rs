use proconio::{input as I, fastout as F, marker::{Chars as C, Usize1 as U}};

#[F]
fn main() {
    I! {
        h: usize,
        w: usize,
        a: [C; h],
        n: usize,
        e: [(U, U, i32); n]
    }
    let mut b = vec![vec![0; w]; h];
    let mut s = (0, 0);
    let mut t = (0, 0);
    for i in 0..h {
        for j in 0..w {
            match a[i][j] {
                'S' => s = (i, j),
                'T' => t = (i, j),
                '#' => b[i][j] = -1,
                _ => ()
            }
        }
    }
    for (r, c, e) in e {
        b[r][c] = e;
    }
    let mut q = std::collections::VecDeque::new();
    let e = b[s.0][s.1];
    let mut f = vec![vec![0; w]; h];
    f[s.0][s.1] += 1;
    q.push_back((s, f, e));
    while let Some(((i, j), f, e)) = q.pop_front() {
        if (i, j) == t {
            println!("Yes");
            return;
        }
        if e <= 0 {continue}

        if i > 0 {
            let (i, j) = (i - 1, j);
            if b[i][j] >= 0 && f[i][j] < 2 {
                let mut f = f.clone();
                let e = if b[i][j] > 0 && f[i][j] == 0 {b[i][j]} else {e - 1};
                f[i][j] += 1;
                q.push_back(((i, j), f, e));
            }
        }
        if j > 0 {
            let (i, j) = (i, j - 1);
            if b[i][j] >= 0 && f[i][j] < 2 {
                let mut f = f.clone();
                let e = if b[i][j] > 0 && f[i][j] == 0 {b[i][j]} else {e - 1};
                f[i][j] += 1;
                q.push_back(((i, j), f, e));
            }
        }
        if i < h - 1 {
            let (i, j) = (i + 1, j);
            if b[i][j] >= 0 && f[i][j] < 2 {
                let mut f = f.clone();
                let e = if b[i][j] > 0 && f[i][j] == 0 {b[i][j]} else {e - 1};
                f[i][j] += 1;
                q.push_back(((i, j), f, e));
            }
        }
        if j < w - 1 {
            let (i, j) = (i, j + 1);
            if b[i][j] >= 0 && f[i][j] < 2 {
                let mut f = f.clone();
                let e = if b[i][j] > 0 && f[i][j] == 0 {b[i][j]} else {e - 1};
                f[i][j] += 1;
                q.push_back(((i, j), f, e));
            }
        }
    }
    println!("No");
}
