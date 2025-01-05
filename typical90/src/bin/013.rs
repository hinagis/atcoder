use proconio::{input as I, fastout as F};
use petgraph::{graph::UnGraph as G, algo::dijkstra as D};

#[F]
fn main() {
    I! {
        n: u32,
        m: usize,
        abc: [(u32, u32, u32); m]
    }

    let g = G::<(), u32>::from_edges(&abc);
    let s = D(&g, 1.into(), None, |e| *e.weight());
    let t = D(&g, n.into(), None, |e| *e.weight());
    for i in 1..=n {
        println!("{}", s[&i.into()] + t[&i.into()]);
    }
}
