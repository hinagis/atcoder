use proconio::{input, marker::Usize1};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        mut p: [Usize1; n],
        m: usize
    }

    let mut e = vec![std::collections::HashMap::new(); n];
    let mut uf = petgraph::unionfind::UnionFind::new(n);
    for i in 0..m {
        input! {a: Usize1, b: Usize1}
        if uf.union(a, b) {
            e[a].insert(b, i);
            e[b].insert(a, i);
        }
    }

    let mut r = vec![];

    let mut l = (0..n).filter(|&i| e[i].len() <= 1).collect_vec();
    while let Some(i) = l.pop() {
        let dfs = |t| -> (Vec<usize>, usize) {
            let mut next = vec![n; n];
            let mut s = vec![t];
            while let Some(i) = s.pop() {
                if p[i] == t {
                    return (next, i);
                }
                for &e in e[i].keys() {
                    if e != t && next[e] == n {
                        next[e] = i;
                        s.push(e);
                    }
                }
            }
            (next, n)
        };
        let (next, f) = dfs(i);
        if f == n {
            println!("{}", -1);
            return;
        }

        let mut mov = |mut i: usize, t| {
            while i != t {
                r.push(*e[i].get(&next[i]).unwrap());
                p.swap(i, next[i]);
                i = next[i];
            }
        };
        mov(f, i);

        for j in e[i].keys().map(|&j| j).collect_vec() {
            e[j].remove(&i);
            if e[j].len() == 1 {
                l.push(j);
            }
        }
    }

    println!("{}\n{}", r.len(), r.iter().map(|&i| i + 1).join(" "));
}
