use proconio::{input as I, marker::{Chars, Usize1 as U1}};

fn main() {
    const D: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

    I! {
        n: isize,
        a: (U1, U1),
        b: (U1, U1),
        s: [Chars; n]
    }

    let mut q = std::collections::VecDeque::new();
    let mut f = vec![vec![[u32::max_value(); 4]; n as usize]; n as usize];
    for i in 0..4 {
        let (x, y) = (a.0 as isize + D[i].0, a.1 as isize + D[i].1);
        if x >= 0 && x < n && y >= 0 && y < n {
            let (x, y) = (x as usize, y as usize);
            if s[x][y] == '.' {
                f[x][y][i] = 1;
                q.push_back(((x, y), i));
            }
        }
    }

    while let Some(((u, v), d)) = q.pop_front() {
        if (u, v) == b {
            println!("{}", f[u][v][d]);
            return;
        }
        for i in 0..4 {
            let (x, y) = (u as isize + D[i].0, v as isize + D[i].1);
            if x >= 0 && x < n && y >= 0 && y < n {
                let (x, y) = (x as usize, y as usize);
                if s[x][y] == '.' {
                    let c = f[u][v][d] + if i == d {0} else {1};
                    if c < f[x][y][i] {
                        f[x][y][i] = c;
                        if i == d {
                            q.push_front(((x, y), i));
                        } else {
                            q.push_back(((x, y), i));
                        }
                    }
                }
            }
        }
    }
    println!("-1");
}
