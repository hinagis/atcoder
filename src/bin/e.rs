fn main() {
    proconio::input! {
        n: usize,
        xyz: [(i32, i32, i32); n],
    }
    let nbit = 2usize.pow(n as u32);
    let mut dp = vec![vec![None; nbit]; n];
    dp[0][0] = Some(0);

    for now in 0..nbit {
        for i in 0..n {
            if let Some(cost) = dp[i][now] {
                let (a, b, c) = xyz[i];
                for j in 0..n {
                    if j != i {
                        let (p, q, r) = xyz[j];
                        let next = now | (1 << j);
                        let d = (p - a).abs() + (q - b).abs() + (r - c).max(0);
                        let dc = cost + d as u64;
                        dp[j][next] = Some(if let Some(nc) = dp[j][next] {nc.min(dc)} else {dc});
                    }
                }
            }
        }
    }
    println!("{}", dp[0][nbit - 1].unwrap());
}
