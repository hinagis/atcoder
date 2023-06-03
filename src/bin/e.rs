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
    for &(x, y) in &m {
        t.union(x, y);
    }

    let mut h = std::collections::HashSet::new();
    for &(x, y) in &k {
        h.insert((t.find(x), t.find(y)));
    }
    for &(x, y) in &q {
        println!("{}", if h.contains(&(t.find(x), t.find(y))) {"No"} else {"Yes"});
    }
}
