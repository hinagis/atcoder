use proconio::{input, marker::{Chars, Usize1}};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: (Usize1, Usize1),
        g: (Usize1, Usize1),
        b: [Chars; h]
    }

    let f = |c: &mut Vec<Vec<_>>, q: &mut VecDeque<_>, y: usize, x, t, d, p| {
        let d = d + p;
        if b[y][x] == '.' && d <= c[y][x] {
            c[y][x] = d;
            if p == 0 {
                q.push_front(((y, x), t, d))
            } else {
                q.push_back(((y, x), t, d))
            }
        }
    };

    let mut c = vec![vec![u32::max_value(); w]; h];
    c[s.0][s.1] = 0;
    let mut q = VecDeque::new();
    q.push_back((s, (0, 0, 0, 0), 0));
    while let Some((s, t, d)) = q.pop_front() {
        if s.0 > 0     {f(&mut c, &mut q, s.0 - 1, s.1, (0, 1, 2, 1), d, t.0)}
        if s.1 > 0     {f(&mut c, &mut q, s.0, s.1 - 1, (1, 0, 1, 2), d, t.1)}
        if s.0 < h - 1 {f(&mut c, &mut q, s.0 + 1, s.1, (2, 1, 0, 1), d, t.2)}
        if s.1 < w - 1 {f(&mut c, &mut q, s.0, s.1 + 1, (1, 2, 1, 0), d, t.3)}
    }
    println!("{}", c[g.0][g.1]);
}
