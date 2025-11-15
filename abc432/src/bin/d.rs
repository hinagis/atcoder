use itertools::Itertools;
use proconio::{input as I, fastout as F};
use petgraph::unionfind;

#[F]
fn main() {
    I! {
        n: usize,
        x: i64,
        y: i64
    }
    let mut g = vec![(0, x - 1, 0, y - 1)];
    for _ in 0..n {
        I! {
            c: char,
            a: i64,
            b: i64
        }
        let mut t = vec![];
        if c == 'X' {
            for (u, x, v, y) in g {
                if u >= a {
                    t.push((u, x, v + b, y + b));
                } else if x < a {
                    t.push((u, x, v - b, y - b));
                } else {
                    t.push((u, a - 1, v - b, y - b));
                    t.push((a, x, v + b, y + b));
                }
            }
        } else {
            for (u, x, v, y) in g {
                if b >= y {
                    t.push((u + b, x + b, v, y));
                } else if b < v {
                    t.push((u - b, x - b, v, y));
                } else {
                    t.push((u - a, x - a, v, b - 1));
                    t.push((u - a, x - a, b, y));
                }
            }
        }
        g = t;
    }
    let mut f = unionfind::UnionFind::new(g.len());
    for i in 0..g.len() {
        for j in i + 1..g.len() {
            if ((g[i].0 == g[j].1 + 1 || g[i].1 + 1 == g[j].0)
              && g[i].2 <= g[j].3
              && g[i].3 >= g[j].2)
            || ((g[i].2 == g[j].3 + 1 || g[i].3 + 1 == g[j].2)
              && g[i].0 <= g[j].1
              && g[i].1 >= g[j].0) {
                f.union(i, j);
            }
        }
    }
    let mut h = std::collections::HashMap::new();
    for i in 0..g.len() {
        *h.entry(f.find(i)).or_insert(0) += ((g[i].1 - g[i].0).abs() + 1) * ((g[i].3 - g[i].2).abs() + 1);
    }

    println!("{}", h.len());
    let mut h = h.values().collect::<Vec<&i64>>();
    h.sort();
    println!("{}", h.iter().join(" "));
}
