const M: u64 = 1000_000_007;
fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }
    let mut t = vec![vec![]; n + 1];
    for i in 0..m {
        let (a, b) = ab[i];
        t[a].push(b);
        t[b].push(a);
    }
    let mut dp = vec![None; n + 1];
    dp[1] = Some((1, 0));
    let mut q = std::collections::VecDeque::new();
    q.push_back(1);
    while let Some(v) = q.pop_front() {
        let (c, d): (u64, u64) = dp[v].unwrap();
        for &i in &t[v] {
            if let Some(u) = &mut dp[i] {
                if u.1 == d + 1 {
                    u.0 = (u.0 + c) % M;
                }
            } else {
                dp[i] = Some((c, d + 1));
                q.push_back(i);
            }
        }
    }
    println!("{}", if let Some((c, _)) = dp[n] {c} else {0});
}
