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
    let mut r = u32::max_value();
    let mut wq = vec![];
    while let Some(c) = q.pop() {
        if c.0 == d.0 && c.1 == d.1 {
            r = r.min(dp[c.0][c.1]);
        } else {
            let v = dp[c.0][c.1];
            if c.0 > 0     && v < dp[c.0 - 1][c.1] && s[c.0 - 1][c.1] == '.' { dp[c.0 - 1][c.1] = v; q.push((c.0 - 1, c.1)); }
            if c.0 + 1 < h && v < dp[c.0 + 1][c.1] && s[c.0 + 1][c.1] == '.' { dp[c.0 + 1][c.1] = v; q.push((c.0 + 1, c.1)); }
            if c.1 > 0     && v < dp[c.0][c.1 - 1] && s[c.0][c.1 - 1] == '.' { dp[c.0][c.1 - 1] = v; q.push((c.0, c.1 - 1)); }
            if c.1 + 1 < w && v < dp[c.0][c.1 + 1] && s[c.0][c.1 + 1] == '.' { dp[c.0][c.1 + 1] = v; q.push((c.0, c.1 + 1)); }

            if c.0 > 1                    && v + 1 < dp[c.0 - 2][c.1    ] && s[c.0 - 2][c.1    ] == '.' { dp[c.0 - 2][c.1    ] = v + 1; wq.push((c.0 - 2, c.1    )); }
            if c.0 > 0     && c.1 > 0     && v + 1 < dp[c.0 - 1][c.1 - 1] && s[c.0 - 1][c.1 - 1] == '.' { dp[c.0 - 1][c.1 - 1] = v + 1; wq.push((c.0 - 1, c.1 - 1)); }
            if                c.1 > 1     && v + 1 < dp[c.0    ][c.1 - 2] && s[c.0    ][c.1 - 2] == '.' { dp[c.0    ][c.1 - 2] = v + 1; wq.push((c.0    , c.1 - 2)); }
            if c.0 + 1 < h && c.1 > 0     && v + 1 < dp[c.0 + 1][c.1 - 1] && s[c.0 + 1][c.1 - 1] == '.' { dp[c.0 + 1][c.1 - 1] = v + 1; wq.push((c.0 + 1, c.1 - 1)); }
            if c.0 + 2 < h                && v + 1 < dp[c.0 + 2][c.1    ] && s[c.0 + 2][c.1    ] == '.' { dp[c.0 + 2][c.1    ] = v + 1; wq.push((c.0 + 2, c.1    )); }
            if c.0 + 1 < h && c.1 + 1 < w && v + 1 < dp[c.0 + 1][c.1 + 1] && s[c.0 + 1][c.1 + 1] == '.' { dp[c.0 + 1][c.1 + 1] = v + 1; wq.push((c.0 + 1, c.1 + 1)); }
            if                c.1 + 2 < w && v + 1 < dp[c.0    ][c.1 + 2] && s[c.0    ][c.1 + 2] == '.' { dp[c.0    ][c.1 + 2] = v + 1; wq.push((c.0    , c.1 + 2)); }
            if c.0 > 0     && c.1 + 1 < w && v + 1 < dp[c.0 - 1][c.1 + 1] && s[c.0 - 1][c.1 + 1] == '.' { dp[c.0 - 1][c.1 + 1] = v + 1; wq.push((c.0 - 1, c.1 + 1)); }

            if c.0 > 1     && c.1 > 1     && v + 1 < dp[c.0 - 2][c.1 - 2] && s[c.0 - 2][c.1 - 2] == '.' { dp[c.0 - 2][c.1 - 2] = v + 1; wq.push((c.0 - 2, c.1 - 2)); }
            if c.0 + 2 < h && c.1 > 1     && v + 1 < dp[c.0 + 2][c.1 - 2] && s[c.0 + 2][c.1 - 2] == '.' { dp[c.0 + 2][c.1 - 2] = v + 1; wq.push((c.0 + 2, c.1 - 2)); }
            if c.0 + 2 < h && c.1 + 2 < w && v + 1 < dp[c.0 + 2][c.1 + 2] && s[c.0 + 2][c.1 + 2] == '.' { dp[c.0 + 2][c.1 + 2] = v + 1; wq.push((c.0 + 2, c.1 + 2)); }
            if c.0 > 1     && c.1 + 2 < w && v + 1 < dp[c.0 - 2][c.1 + 2] && s[c.0 - 2][c.1 + 2] == '.' { dp[c.0 - 2][c.1 + 2] = v + 1; wq.push((c.0 - 2, c.1 + 2)); }
        }
        while q.is_empty() && !wq.is_empty() {
            let c = wq.pop().unwrap();
            if c.0 == d.0 && c.1 == d.1 {
                r = r.min(dp[c.0][c.1]);
            } else {
                let v = dp[c.0][c.1];
                if c.0 > 0     && v < dp[c.0 - 1][c.1] && s[c.0 - 1][c.1] == '.' { dp[c.0 - 1][c.1] = v; q.push((c.0 - 1, c.1)); }
                if c.0 + 1 < h && v < dp[c.0 + 1][c.1] && s[c.0 + 1][c.1] == '.' { dp[c.0 + 1][c.1] = v; q.push((c.0 + 1, c.1)); }
                if c.1 > 0     && v < dp[c.0][c.1 - 1] && s[c.0][c.1 - 1] == '.' { dp[c.0][c.1 - 1] = v; q.push((c.0, c.1 - 1)); }
                if c.1 + 1 < w && v < dp[c.0][c.1 + 1] && s[c.0][c.1 + 1] == '.' { dp[c.0][c.1 + 1] = v; q.push((c.0, c.1 + 1)); }
    
                if c.0 > 1                    && v + 1 < dp[c.0 - 2][c.1    ] && s[c.0 - 2][c.1    ] == '.' { dp[c.0 - 2][c.1    ] = v + 1; wq.push((c.0 - 2, c.1    )); }
                if c.0 > 0     && c.1 > 0     && v + 1 < dp[c.0 - 1][c.1 - 1] && s[c.0 - 1][c.1 - 1] == '.' { dp[c.0 - 1][c.1 - 1] = v + 1; wq.push((c.0 - 1, c.1 - 1)); }
                if                c.1 > 1     && v + 1 < dp[c.0    ][c.1 - 2] && s[c.0    ][c.1 - 2] == '.' { dp[c.0    ][c.1 - 2] = v + 1; wq.push((c.0    , c.1 - 2)); }
                if c.0 + 1 < h && c.1 > 0     && v + 1 < dp[c.0 + 1][c.1 - 1] && s[c.0 + 1][c.1 - 1] == '.' { dp[c.0 + 1][c.1 - 1] = v + 1; wq.push((c.0 + 1, c.1 - 1)); }
                if c.0 + 2 < h                && v + 1 < dp[c.0 + 2][c.1    ] && s[c.0 + 2][c.1    ] == '.' { dp[c.0 + 2][c.1    ] = v + 1; wq.push((c.0 + 2, c.1    )); }
                if c.0 + 1 < h && c.1 + 1 < w && v + 1 < dp[c.0 + 1][c.1 + 1] && s[c.0 + 1][c.1 + 1] == '.' { dp[c.0 + 1][c.1 + 1] = v + 1; wq.push((c.0 + 1, c.1 + 1)); }
                if                c.1 + 2 < w && v + 1 < dp[c.0    ][c.1 + 2] && s[c.0    ][c.1 + 2] == '.' { dp[c.0    ][c.1 + 2] = v + 1; wq.push((c.0    , c.1 + 2)); }
                if c.0 > 0     && c.1 + 1 < w && v + 1 < dp[c.0 - 1][c.1 + 1] && s[c.0 - 1][c.1 + 1] == '.' { dp[c.0 - 1][c.1 + 1] = v + 1; wq.push((c.0 - 1, c.1 + 1)); }

                if c.0 > 1     && c.1 > 1     && v + 1 < dp[c.0 - 2][c.1 - 2] && s[c.0 - 2][c.1 - 2] == '.' { dp[c.0 - 2][c.1 - 2] = v + 1; wq.push((c.0 - 2, c.1 - 2)); }
                if c.0 + 2 < h && c.1 > 1     && v + 1 < dp[c.0 + 2][c.1 - 2] && s[c.0 + 2][c.1 - 2] == '.' { dp[c.0 + 2][c.1 - 2] = v + 1; wq.push((c.0 + 2, c.1 - 2)); }
                if c.0 + 2 < h && c.1 + 2 < w && v + 1 < dp[c.0 + 2][c.1 + 2] && s[c.0 + 2][c.1 + 2] == '.' { dp[c.0 + 2][c.1 + 2] = v + 1; wq.push((c.0 + 2, c.1 + 2)); }
                if c.0 > 1     && c.1 + 2 < w && v + 1 < dp[c.0 - 2][c.1 + 2] && s[c.0 - 2][c.1 + 2] == '.' { dp[c.0 - 2][c.1 + 2] = v + 1; wq.push((c.0 - 2, c.1 + 2)); }
            }
        }
    }

    if dp[d.0][d.1] < u32::max_value() {
        println!("{}", dp[d.0][d.1]);
    } else {
        println!("-1");
    }
}
