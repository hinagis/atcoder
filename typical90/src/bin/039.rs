use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        ab: [(U, U); n - 1]
    }

    let mut t = vec![vec![]; n];
    for &(a, b) in &ab {
        t[a].push(b);
        t[b].push(a);
    }

    let mut dp = vec![0; n];
    dfs(&mut dp, &t, 0, 0);

    let mut r = 0;
    for &(a, b) in &ab {
        let v = dp[a].min(dp[b]);
        r += v * (n as u64 - v);
    }

    println!("{}", r);
}

fn dfs(dp: &mut Vec<u64>, t: &Vec<Vec<usize>>, i: usize, p: usize) {
    dp[i] = 1;
    for &j in &t[i] {
        if j == p {continue}
        dfs(dp, t, j, i);
        dp[i] += dp[j];
    }
}
