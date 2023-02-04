use petgraph::{graph::UnGraph as G, algo::tarjan_scc as S};

fn main() {
    proconio::input! {
        _: u32,
        m: usize,
        ab: [(u32, u32); m]
    }

    let g = G::<(), ()>::from_edges(&ab);
    let s = S(&g);
    println!("{}", m - s.iter().fold(0, |s, e| s + e.len() - 1));
}
