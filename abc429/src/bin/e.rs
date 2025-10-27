use proconio::{input as I, fastout as F, marker::{Chars as C, Usize1 as U}};

#[F]
fn main() {
    I! {
        n: usize,
        m: usize,
        e: [(U, U); m],
        s: C,
    }

    let mut g = vec![vec![]; n];
    for (u, v) in e {
        g[u].push(v);
        g[v].push(u);
    }

    let mut dp = vec![Vec::with_capacity(2); n];
    let mut q = std::collections::VecDeque::new();

    for (i, &c) in s.iter().enumerate() {
        if c == 'S' {
            dp[i].push((0, i));
            q.push_back((i, 0, i));
        }
    }

    while let Some((i, l, u)) = q.pop_front() {
        for &v in &g[i] {
            if dp[v].len() < 2 && dp[v].iter().all(|&(_, j)| j != u) {
                dp[v].push((l + 1, u));
                q.push_back((v, l + 1, u));
            }
        }
    }

    for i in 0..n {
        if s[i] == 'D' {
            println!("{}", dp[i][0].0 + dp[i][1].0);
        }
    }
}
