use petgraph::unionfind::UnionFind as G;

fn main() {
    proconio::input! {
        n: usize,
        p: [(i32, i32); n]
    }

    let mut g = G::new(n);

    for i in 0..n {
        let (x, y) = p[i];
        for j in 0..n {
            let (u, v) = p[j];
            if (x - 1 == u && (y - 1 == v || y     == v))
            || (x     == u && (y - 1 == v || y + 1 == v))
            || (x + 1 == u && (y     == v || y + 1 == v)) {
                g.union(i, j);
            }
        }
    }
    let mut h = std::collections::HashSet::new();
    for i in 0..n {
        h.insert(g.find(i));
    }
    println!("{}", h.len());
}
