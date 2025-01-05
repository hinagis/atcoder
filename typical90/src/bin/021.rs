use petgraph::{graph::Graph as G, algo::tarjan_scc as S};

fn main() {
    proconio::input! {
        _: u32,
        m: usize,
        ab: [(u32, u32); m]
    }

    let g = G::<(), ()>::from_edges(&ab);
    let s = S(&g);
    println!("{}", s.iter().fold(0, |s, e| s + e.len() * (e.len() - 1) / 2));
}
