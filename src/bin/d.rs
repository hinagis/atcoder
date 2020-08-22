use proconio::{input, marker::{Usize1, Chars}};
fn main() {
    input! {
        h: usize,
        w: usize,
        c: (Usize1, Usize1),
        d: (Usize1, Usize1),
        s: [Chars; h],
    }

    let mut dp = vec![vec![false; w]; h];
    dp[c.0][c.1] = true;
    let mut q = vec![(c.0, c.1, 0)];
    let mut r = None;
    let mut wq = vec![];
    while let Some(c) = q.pop() {
        if c.0 == d.0 && c.1 == d.1 {
            r = Some(c.2);
            break;
        } else {
            let mut f = false;
            if c.0 > 0     && dp[c.0 - 1][c.1] && s[c.0 - 1][c.1] == '.' { f = true; dp[c.0 - 1][c.1] = true; q.push((c.0 - 1, c.1, c.2)); }
            if c.0 + 1 < h && dp[c.0 + 1][c.1] && s[c.0 + 1][c.1] == '.' { f = true; dp[c.0 + 1][c.1] = true; q.push((c.0 + 1, c.1, c.2)); }
            if c.1 > 0     && dp[c.0][c.1 - 1] && s[c.0][c.1 - 1] == '.' { f = true; dp[c.0][c.1 - 1] = true; q.push((c.0, c.1 - 1, c.2)); }
            if c.1 + 1 < w && dp[c.0][c.1 + 1] && s[c.0][c.1 + 1] == '.' { f = true; dp[c.0][c.1 + 1] = true; q.push((c.0, c.1 + 1, c.2)); }
            if !f {
                if c.0 > 1                    && dp[c.0 - 2][c.1    ] && s[c.0 - 2][c.1    ] == '.' { dp[c.0 - 2][c.1    ] = true; wq.push((c.0 - 2, c.1    , c.2 + 1)); }
                if c.0 > 0     && c.1 > 0     && dp[c.0 - 1][c.1 - 1] && s[c.0 - 1][c.1 - 1] == '.' { dp[c.0 - 1][c.1 - 1] = true; wq.push((c.0 - 1, c.1 - 1, c.2 + 1)); }
                if                c.1 > 1     && dp[c.0    ][c.1 - 2] && s[c.0    ][c.1 - 2] == '.' { dp[c.0    ][c.1 - 2] = true; wq.push((c.0    , c.1 - 2, c.2 + 1)); }
                if c.0 + 1 < h && c.1 > 0     && dp[c.0 + 1][c.1 - 1] && s[c.0 + 1][c.1 - 1] == '.' { dp[c.0 + 1][c.1 - 1] = true; wq.push((c.0 + 1, c.1 - 1, c.2 + 1)); }
                if c.0 + 2 < h                && dp[c.0 + 2][c.1    ] && s[c.0 + 2][c.1    ] == '.' { dp[c.0 + 2][c.1    ] = true; wq.push((c.0 + 2, c.1    , c.2 + 1)); }
                if c.0 + 1 < h && c.1 + 1 < w && dp[c.0 + 1][c.1 + 1] && s[c.0 + 1][c.1 + 1] == '.' { dp[c.0 + 1][c.1 + 1] = true; wq.push((c.0 + 1, c.1 + 1, c.2 + 1)); }
                if                c.1 + 2 < w && dp[c.0    ][c.1 + 2] && s[c.0    ][c.1 + 2] == '.' { dp[c.0    ][c.1 + 2] = true; wq.push((c.0    , c.1 + 2, c.2 + 1)); }
                if c.0 > 0     && c.1 + 1 < w && dp[c.0 - 1][c.1 + 1] && s[c.0 - 1][c.1 + 1] == '.' { dp[c.0 - 1][c.1 + 1] = true; wq.push((c.0 - 1, c.1 + 1, c.2 + 1)); }
            }
        }
        while q.is_empty() && !wq.is_empty() {
            let c = wq.pop().unwrap();
            if c.0 > 0     && dp[c.0 - 1][c.1] && s[c.0 - 1][c.1] == '.' { dp[c.0 - 1][c.1] = true; q.push((c.0 - 1, c.1, c.2)); }
            if c.0 + 1 < h && dp[c.0 + 1][c.1] && s[c.0 + 1][c.1] == '.' { dp[c.0 + 1][c.1] = true; q.push((c.0 + 1, c.1, c.2)); }
            if c.1 > 0     && dp[c.0][c.1 - 1] && s[c.0][c.1 - 1] == '.' { dp[c.0][c.1 - 1] = true; q.push((c.0, c.1 - 1, c.2)); }
            if c.1 + 1 < w && dp[c.0][c.1 + 1] && s[c.0][c.1 + 1] == '.' { dp[c.0][c.1 + 1] = true; q.push((c.0, c.1 + 1, c.2)); }
        }
    }

    if let Some(r) = r {
        println!("{}", r);
    } else {
        println!("-1");
    }
}
