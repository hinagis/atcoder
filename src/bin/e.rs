use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
        q: usize,
        tex: [(u8, Usize1, u64); q]
    }
    let mut g = vec![Vec::new(); n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }

    let mut c = vec![0; n];
    for &(t, e, x) in &tex {
        let (a, b) = if t == 1 {ab[e]} else {(ab[e].1, ab[e].0)};
        let mut done = std::collections::HashSet::new();
        done.insert(a);
        let mut next = vec![a];
        c[a] += x;
        while !next.is_empty() {
            let now = next;
            next = vec![];
            for i in now {
                for &j in &g[i] {
                    if j != b && !done.contains(&j) {
                        c[j] += x;
                        done.insert(j);
                        next.push(j);
                    }
                }
            }
        }
    }
    for c in c {
        println!("{}", c);
    }
}
