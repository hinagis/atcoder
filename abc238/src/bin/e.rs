use petgraph::unionfind::UnionFind;

fn main() {
    proconio::input! {
        n: usize,
        q: usize,
        lr: [(usize, usize); q],
    }

    let mut g = UnionFind::new(n + 1);
    for &(l, r) in &lr {
        g.union(l - 1, r);
    }

    println!("{}", if g.find(0) == g.find(n) {"Yes"} else {"No"});
}
