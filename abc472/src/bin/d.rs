use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        h: usize,
        w: usize,
        k: usize,
        s: [C; h]
    }
    let mut b = (vec![true; h], vec![true; w]);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                b.0[i] = false;
                b.1[j] = false;
            }
        }
    }

    let mut c = vec![vec![usize::MAX; w]; h];
    let mut q = std::collections::VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if b.0[i] && b.1[j] {
                c[i][j] = 0;
                q.push_back((i, j));
            }
        }
    }

    while let Some((i, j)) = q.pop_front() {
        let nc = c[i][j] + 1;
        if nc > k {
            continue;
        }
        for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (ni, nj) = (i as isize + di, j as isize + dj);
            if ni < 0 || ni >= h as isize || nj < 0 || nj >= w as isize {
                continue;
            }
            let (ni, nj) = (ni as usize, nj as usize);
            if s[ni][nj] == '#' {
                continue;
            }
            if c[ni][nj] > nc {
                c[ni][nj] = nc;
                q.push_back((ni, nj));
            }
        }
    }
    println!("{}", c.iter().map(|r| r.iter().filter(|&&x| x <= k).count()).sum::<usize>());
}
