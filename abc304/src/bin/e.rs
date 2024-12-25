use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        m: [(U, U)],
        k: [(U, U)],
        q: [(U, U)]
    }

    let mut t = petgraph::unionfind::UnionFind::new(n);
    for (x, y) in m {
        t.union(x, y);
    }
    let g = t.into_labeling();

    let mut h = std::collections::HashSet::new();
    for (x, y) in k {
        h.insert((g[x].min(g[y]), g[y].max(g[x])));
    }

    for (x, y) in q {
        println!("{}", if h.contains(&(g[x].min(g[y]), g[y].max(g[x]))) {"No"} else {"Yes"});
    }
}
