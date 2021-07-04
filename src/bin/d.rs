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
            let mut f = vec![false; n + 1];
            f[t] = true;
            dfs(&d, &mut kts[k], &mut f, k, t, t, 0);
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
    dbg!(&d);
    println!("{}", r);
}

fn dfs(d: &Vec<HashMap<usize, u64>>, ts: &mut Vec<Vec<u64>>, f: &mut Vec<bool>, k: usize, t: usize, b: usize, c: u64) {
    for &a in d[b].keys() {
        let c = c + d[b].get(&a).unwrap();
        ts[t][a] = if ts[t][a] == 0 {c} else {ts[t][a].min(c)};
        if ! f[a] {
            f[a] = true;
            if a <= k {
                dfs(d, ts, f, k, t, a, c);
            }
        }
    }
}