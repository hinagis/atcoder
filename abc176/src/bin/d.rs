use proconio::{input, marker::{Usize1, Chars}};
fn main() {
    input! {
        h: usize,
        w: usize,
        c: (Usize1, Usize1),
        d: (Usize1, Usize1),
        s: [Chars; h],
    }

    let mut dp = vec![vec![u32::max_value(); w]; h];
    dp[c.0][c.1] = 0;
    let mut q = vec![(c.0, c.1)];
    let mut v = 0;
    while !q.is_empty() {
        q = move_a(&mut dp, &mut q, v, &s, h, w);
        if !q.is_empty() {
            v += 1;
            q = move_b(&mut dp, &q, v, &s, h, w);
        }
    }

    if dp[d.0][d.1] < u32::max_value() {
        println!("{}", dp[d.0][d.1]);
    } else {
        println!("-1");
    }
}

fn move_a(
    dp: &mut Vec<Vec<u32>>,
    q: &mut Vec<(usize, usize)>,
    v: u32,
    s: &Vec<Vec<char>>,
    h: usize,
    w: usize,
) -> Vec<(usize, usize)> {
    let mut nq = vec![];
    while let Some(c) = q.pop() {
        nq.push(c);
        if c.0 > 0     { let c = (c.0 - 1, c.1); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; q.push(c); } }
        if c.0 + 1 < h { let c = (c.0 + 1, c.1); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; q.push(c); } }
        if c.1 > 0     { let c = (c.0, c.1 - 1); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; q.push(c); } }
        if c.1 + 1 < w { let c = (c.0, c.1 + 1); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; q.push(c); } }
    }
    nq
}

fn move_b(
    dp: &mut Vec<Vec<u32>>,
    q: &Vec<(usize, usize)>,
    v: u32,
    s: &Vec<Vec<char>>,
    h: usize,
    w: usize,
) -> Vec<(usize, usize)> {
    let mut nq = vec![];
    for c in q {
        if c.0 > 1     && c.1 > 1     { let c = (c.0 - 2, c.1 - 2); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; nq.push(c); } }
        if c.0 > 1     && c.1 > 0     { let c = (c.0 - 2, c.1 - 1); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; nq.push(c); } }
        if c.0 > 1                    { let c = (c.0 - 2, c.1    ); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; nq.push(c); } }
        if c.0 > 1     && c.1 + 1 < w { let c = (c.0 - 2, c.1 + 1); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; nq.push(c); } }
        if c.0 > 1     && c.1 + 2 < w { let c = (c.0 - 2, c.1 + 2); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; nq.push(c); } }

        if c.0 > 0     && c.1 > 1     { let c = (c.0 - 1, c.1 - 2); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; nq.push(c); } }
        if c.0 > 0     && c.1 > 0     { let c = (c.0 - 1, c.1 - 1); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; nq.push(c); } }
        if c.0 > 0     && c.1 + 1 < w { let c = (c.0 - 1, c.1 + 1); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; nq.push(c); } }
        if c.0 > 0     && c.1 + 2 < w { let c = (c.0 - 1, c.1 + 2); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; nq.push(c); } }

        if                c.1 > 1     { let c = (c.0    , c.1 - 2); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; nq.push(c); } }
        if                c.1 + 2 < w { let c = (c.0    , c.1 + 2); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; nq.push(c); } }

        if c.0 + 1 < h && c.1 > 1     { let c = (c.0 + 1, c.1 - 2); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; nq.push(c); } }
        if c.0 + 1 < h && c.1 > 0     { let c = (c.0 + 1, c.1 - 1); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; nq.push(c); } }
        if c.0 + 1 < h && c.1 + 1 < w { let c = (c.0 + 1, c.1 + 1); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; nq.push(c); } }
        if c.0 + 1 < h && c.1 + 2 < w { let c = (c.0 + 1, c.1 + 2); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; nq.push(c); } }

        if c.0 + 2 < h && c.1 > 1     { let c = (c.0 + 2, c.1 - 2); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; nq.push(c); } }
        if c.0 + 2 < h && c.1 > 0     { let c = (c.0 + 2, c.1 - 1); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; nq.push(c); } }
        if c.0 + 2 < h                { let c = (c.0 + 2, c.1    ); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; nq.push(c); } }
        if c.0 + 2 < h && c.1 + 1 < w { let c = (c.0 + 2, c.1 + 1); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; nq.push(c); } }
        if c.0 + 2 < h && c.1 + 2 < w { let c = (c.0 + 2, c.1 + 2); if v < dp[c.0][c.1] && s[c.0][c.1] == '.' { dp[c.0][c.1] = v; nq.push(c); } }
    }
    nq
}