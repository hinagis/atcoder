use petgraph::{algo::dijkstra, prelude::*};
use proconio::{input, marker::Usize1 as U1};

fn main() {
    input! {
        n: usize,
        m: usize,
        h: [i64; n],
        uv: [(U1, U1); m],
    }

    let mut g: UnGraph<i64, ()> = Graph::new_undirected();
    let p: Vec<_> = h.iter().map(|&h| g.add_node(h)).collect();
    g.extend_with_edges(uv.iter().map(|&(u, v)| (p[u], p[v])));

    println!("{}", h[0] - dijkstra(&g, p[0], None, |e| {
            (g[e.target()] - g[e.source()]).max(0)
        })
        .into_iter()
        .map(|(n, c)| g[n] + c)
        .min()
        .unwrap()
    );
}
