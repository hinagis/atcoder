use std::collections::HashMap;

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, u64); m]
    }
    let mut d = vec![HashMap::new(); n + 1];
    for i in 0..m {
        let (a, b, c) = abc[i];
        d[b].insert(a, c);
    }

    let mut kts = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
    for k in 1..=n {
        for t in 1..=n {
            dfs(&d, &mut kts[k], k, t, t, 0);
        }
    }

    let mut r = 0;
    for k in 1..=n {
        for t in 1..=n {
            for s in 1..=n {
                r += kts[k][t][s];
            }
        }
    }
    println!("{}", r);
}

fn dfs(d: &Vec<HashMap<usize, u64>>, ts: &mut Vec<Vec<u64>>, k: usize, t: usize, b: usize, c: u64) {
    for &a in d[b].keys() {
        let c = c + d[b].get(&a).unwrap();
        if a != t {
            if ts[t][a] == 0 || ts[t][a] > c {
                ts[t][a] = if ts[t][a] == 0 {c} else {ts[t][a].min(c)};
                if a <= k {
                    dfs(d, ts, k, t, a, c);
                }
            }
        }
    }
}