use proconio::{input as I, marker::Usize1 as U};
use petgraph::{Graph as G, Directed as D, algo::tarjan_scc as scc};

fn main() {
    I! {a: [U]}

    let c = a.iter().enumerate().filter(|&(i, &a)| i == a).count();
    let g = G::<(), (), D, _>::from_edges(
        a.iter().enumerate().map(|(i, &a)| (i, a))
    );

    println!("{}", c + scc(&g).iter().filter(|c| c.len() >= 2).fold(0, |s, c| s + c.len()));
}
